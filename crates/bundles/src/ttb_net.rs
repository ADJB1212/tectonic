// Copyright 2023-2024 the Tectonic Project
// Licensed under the MIT License.

//! Read ttb v1 bundles on the internet.
//!
//! The main type offered by this module is the [`TTBNetBundle`] struct,
//! which can (but should not) be used directly as a [`tectonic_io_base::IoProvider`].
//!
//! Instead, wrap it in a [`crate::BundleCache`] for filesystem-backed caching.

use crate::{
    Bundle, CachableBundle, FileIndex, FileInfo, NET_RETRY_ATTEMPTS, NET_RETRY_SLEEP_MS, batch,
    ttb::{TTBFileIndex, TTBFileInfo, TTBv1Header},
};
use flate2::read::GzDecoder;
use std::{
    convert::TryFrom,
    io::{Cursor, Read},
    thread,
    time::Duration,
};
use tectonic_errors::prelude::*;
use tectonic_geturl::{DefaultBackend, DefaultRangeReader, GetUrlBackend, RangeReader};
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult};
use tectonic_status_base::{StatusBackend, tt_note, tt_warning};

/// Read a [`TTBFileInfo`] from this bundle.
/// We assume that `fileinfo` points to a valid file in this bundle.
fn read_fileinfo(fileinfo: &TTBFileInfo, reader: &mut DefaultRangeReader) -> Result<Box<dyn Read>> {
    // fileinfo.length is a u32, so it must fit inside a usize (assuming 32/64-bit machine).
    let stream = reader.read_range(fileinfo.start, fileinfo.gzip_len as usize)?;
    Ok(Box::new(GzDecoder::new(stream)))
}

fn read_file_with_retries(
    reader: &mut DefaultRangeReader,
    info: &TTBFileInfo,
    status: &mut dyn StatusBackend,
) -> OpenResult<Vec<u8>> {
    if info.gzip_len == 0 {
        return OpenResult::Ok(Vec::new());
    }

    let mut bytes = Vec::with_capacity(info.real_len as usize);
    for attempt in 0..NET_RETRY_ATTEMPTS {
        if attempt > 0 {
            thread::sleep(Duration::from_millis(NET_RETRY_SLEEP_MS));
        }
        bytes.clear();
        let stream = match read_fileinfo(info, reader) {
            Ok(stream) => stream,
            Err(e) => {
                tt_warning!(status,
                    "failure fetching \"{}\" from network ({}/{NET_RETRY_ATTEMPTS})",
                    info.name, attempt + 1; e
                );
                continue;
            }
        };

        match stream
            .take(u64::from(info.real_len) + 1)
            .read_to_end(&mut bytes)
        {
            Ok(_) if bytes.len() == info.real_len as usize => return OpenResult::Ok(bytes),
            result => {
                let error = match result {
                    Err(e) => e.into(),
                    Ok(_) => anyhow!("unexpected uncompressed length for {}", info.name),
                };
                tt_warning!(status,
                    "failure downloading \"{}\" from network ({}/{NET_RETRY_ATTEMPTS})",
                    info.name, attempt + 1; error
                );
            }
        }
    }

    OpenResult::Err(anyhow!(
        "failed to download \"{}\"; please check your network connection.",
        info.name
    ))
}

/// Access ttbv1 bundle hosted on the internet.
/// This struct provides NO caching. All files
/// are downloaded.
///
/// As such, this bundle should probably be wrapped in a [`crate::BundleCache`].
pub struct TTBNetBundle<T>
where
    for<'a> T: FileIndex<'a>,
{
    url: String,
    index: T,

    // We need the network to load these.
    // They're None until absolutely necessary.
    reader: Option<DefaultRangeReader>,
    header: Option<TTBv1Header>,
}

/// The internal file-information struct used by the [`TTBNetBundle`].
impl TTBNetBundle<TTBFileIndex> {
    /// Create a new ZIP bundle for a generic readable and seekable stream.
    /// This method does not require network access.
    /// It will succeed even in we can't connect to the bundle, or if we're given a bad url.
    pub fn new(url: String) -> Result<Self> {
        Ok(TTBNetBundle {
            reader: None,
            header: None,
            index: TTBFileIndex::default(),
            url,
        })
    }

    fn connect_reader(&mut self) -> Result<()> {
        if self.reader.is_some() {
            return Ok(());
        }
        let geturl_backend = DefaultBackend::default();
        self.reader = Some(geturl_backend.open_range_reader(&self.url));
        Ok(())
    }

    fn get_header(&mut self) -> Result<TTBv1Header> {
        if let Some(header) = self.header {
            return Ok(header);
        }
        self.connect_reader()?;
        let mut header: [u8; 70] = [0u8; 70];
        self.reader
            .as_mut()
            .unwrap()
            .read_range(0, 70)?
            .read_exact(&mut header)?;
        let header = TTBv1Header::try_from(header)?;
        self.header = Some(header);
        Ok(header)
    }

    // Fill this bundle's index if it is empty.
    fn ensure_index(&mut self) -> Result<()> {
        if self.index.is_initialized() {
            return Ok(());
        }

        let mut reader = self.get_index_reader()?;
        self.index.initialize(&mut reader)?;
        Ok(())
    }
}

impl IoProvider for TTBNetBundle<TTBFileIndex> {
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        if let Err(e) = self.ensure_index() {
            return OpenResult::Err(e);
        };

        let info = match self.search(name) {
            None => return OpenResult::NotAvailable,
            Some(s) => s,
        };

        // Retries are handled in open_fileinfo,
        // since BundleCache never calls input_open_name.
        self.open_fileinfo(&info, status)
    }
}

impl Bundle for TTBNetBundle<TTBFileIndex> {
    fn all_files(&self) -> Vec<String> {
        self.index.iter().map(|x| x.path().to_owned()).collect()
    }

    fn get_digest(&mut self) -> Result<tectonic_io_base::digest::DigestData> {
        let header = self.get_header()?;
        Ok(header.digest)
    }
}

impl CachableBundle<'_, TTBFileIndex> for TTBNetBundle<TTBFileIndex> {
    fn supports_batch_open(&self) -> bool {
        true
    }

    fn get_location(&mut self) -> String {
        self.url.clone()
    }

    fn initialize_index(&mut self, source: &mut dyn Read) -> Result<()> {
        self.index.initialize(source)?;
        Ok(())
    }

    fn index(&mut self) -> &mut TTBFileIndex {
        &mut self.index
    }

    fn search(&mut self, name: &str) -> Option<TTBFileInfo> {
        self.index.search(name)
    }

    fn get_index_reader(&mut self) -> Result<Box<dyn Read>> {
        self.connect_reader()?;
        let header = self.get_header()?;

        read_fileinfo(
            &TTBFileInfo {
                start: header.index_start,
                gzip_len: header.index_gzip_len,
                real_len: header.index_real_len,
                path: "".to_owned(),
                name: "".to_owned(),
                hash: None,
            },
            self.reader.as_mut().unwrap(),
        )
    }

    fn open_fileinfo(
        &mut self,
        info: &TTBFileInfo,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        tt_note!(status, "downloading {}", info.name);

        if let Err(e) = self.connect_reader() {
            return OpenResult::Err(e);
        }
        match read_file_with_retries(self.reader.as_mut().unwrap(), info, status) {
            OpenResult::Ok(bytes) => OpenResult::Ok(InputHandle::new_read_only(
                info.name.clone(),
                Cursor::new(bytes),
                InputOrigin::Other,
            )),
            OpenResult::NotAvailable => OpenResult::NotAvailable,
            OpenResult::Err(e) => OpenResult::Err(e),
        }
    }

    fn batch_open(
        &mut self,
        infos: &[TTBFileInfo],
        status: &mut dyn StatusBackend,
    ) -> Vec<OpenResult<Vec<u8>>> {
        batch::open(
            &self.url,
            infos,
            |info| (info.real_len as usize).saturating_add(info.gzip_len as usize),
            read_file_with_retries,
            status,
        )
    }
}

#[cfg(all(test, any(feature = "geturl-reqwest", feature = "geturl-curl")))]
mod tests {
    use super::*;
    use flate2::{Compression, write::GzEncoder};
    use std::{
        io::{BufRead, BufReader, Write},
        net::TcpListener,
        time::Instant,
    };
    use tectonic_status_base::NoopStatusBackend;

    fn serve_files(
        files: Vec<(u64, Vec<u8>)>,
        request_count: usize,
    ) -> (String, thread::JoinHandle<Vec<u64>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}/bundle.ttb", listener.local_addr().unwrap());
        let server = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(10);
            let mut ranges = Vec::new();
            for _ in 0..request_count {
                let mut stream = loop {
                    match listener.accept() {
                        Ok((stream, _)) => break stream,
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            assert!(Instant::now() < deadline, "timed out waiting for request");
                            thread::sleep(Duration::from_millis(10));
                        }
                        Err(e) => panic!("accept failed: {e}"),
                    }
                };
                stream.set_nonblocking(false).unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let mut reader = BufReader::new(&stream);
                let mut range = None;
                loop {
                    let mut line = String::new();
                    assert_ne!(reader.read_line(&mut line).unwrap(), 0);
                    if line == "\r\n" {
                        break;
                    }
                    if let Some(value) = line.to_ascii_lowercase().strip_prefix("range: bytes=") {
                        let (start, end) = value.trim().split_once('-').unwrap();
                        range = Some((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
                    }
                }
                let (start, end) = range.unwrap();
                ranges.push(start);
                let (_, body) = files.iter().find(|(offset, _)| *offset == start).unwrap();
                assert_eq!(end - start + 1, body.len() as u64);
                write!(
                    stream,
                    "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )
                .unwrap();
                stream.write_all(body).unwrap();
            }
            ranges
        });
        (url, server)
    }

    fn compressed_file(contents: &[u8], start: u64) -> (TTBFileInfo, Vec<u8>) {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(contents).unwrap();
        let compressed = encoder.finish().unwrap();
        let info = TTBFileInfo {
            start,
            real_len: contents.len() as u32,
            gzip_len: compressed.len() as u32,
            name: format!("file-{start}.tex"),
            path: format!("tex/file-{start}.tex"),
            hash: None,
        };
        (info, compressed)
    }

    #[test]
    fn batch_downloads_and_decompresses_files_in_input_order() {
        let contents = [b"first file".as_slice(), b"second file with more contents"];
        let (first, first_gzip) = compressed_file(contents[0], 100);
        let (second, second_gzip) = compressed_file(contents[1], 1000);
        let empty = TTBFileInfo {
            gzip_len: 0,
            real_len: 0,
            ..first.clone()
        };
        let oversized = TTBFileInfo {
            real_len: u32::MAX,
            ..first.clone()
        };
        let (url, server) = serve_files(vec![(100, first_gzip), (1000, second_gzip)], 2);
        let mut bundle = TTBNetBundle::new(url).unwrap();
        assert!(bundle.supports_batch_open());
        assert!(bundle.batch_open(&[], &mut NoopStatusBackend {}).is_empty());

        let results = bundle.batch_open(
            &[second, empty, oversized, first],
            &mut NoopStatusBackend {},
        );
        let mut results = results.into_iter();
        assert_eq!(results.next().unwrap().unwrap(), contents[1]);
        assert!(results.next().unwrap().unwrap().is_empty());
        assert!(matches!(results.next().unwrap(), OpenResult::NotAvailable));
        assert_eq!(results.next().unwrap().unwrap(), contents[0]);
        assert!(results.next().is_none());
        let mut ranges = server.join().unwrap();
        ranges.sort_unstable();
        assert_eq!(ranges, vec![100, 1000]);
    }

    #[test]
    fn rejects_incorrect_uncompressed_lengths_after_retries() {
        for real_len in [0, 100] {
            let (mut info, compressed) = compressed_file(b"file contents", 100);
            info.real_len = real_len;
            let (url, server) = serve_files(vec![(100, compressed)], NET_RETRY_ATTEMPTS);
            let mut bundle = TTBNetBundle::new(url).unwrap();
            assert!(matches!(
                bundle.open_fileinfo(&info, &mut NoopStatusBackend {}),
                OpenResult::Err(_)
            ));
            assert_eq!(server.join().unwrap(), vec![100; NET_RETRY_ATTEMPTS]);
        }
    }

    #[test]
    fn failed_prefetch_can_be_retried_on_demand() {
        let contents = b"file contents";
        let (mut info, compressed) = compressed_file(contents, 100);
        info.real_len = 0;
        let (url, server) = serve_files(vec![(100, compressed)], NET_RETRY_ATTEMPTS + 1);
        let mut bundle = TTBNetBundle::new(url).unwrap();
        let results = bundle.batch_open(std::slice::from_ref(&info), &mut NoopStatusBackend {});
        assert!(matches!(results.as_slice(), [OpenResult::Err(_)]));

        info.real_len = contents.len() as u32;
        let mut actual = Vec::new();
        bundle
            .open_fileinfo(&info, &mut NoopStatusBackend {})
            .unwrap()
            .read_to_end(&mut actual)
            .unwrap();
        assert_eq!(actual, contents);
        assert_eq!(server.join().unwrap(), vec![100; NET_RETRY_ATTEMPTS + 1]);
    }

    #[test]
    fn reuses_header_and_discards_partial_download_before_retry() {
        let contents = b"bundle file contents";
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(contents).unwrap();
        let compressed = encoder.finish().unwrap();
        let mut header = vec![0; 70];
        header[..14].copy_from_slice(b"tectonicbundle");
        header[14..18].copy_from_slice(&1u32.to_le_bytes());

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}/bundle.ttb", listener.local_addr().unwrap());
        let responses = [
            header,
            compressed[..compressed.len() - 4].to_vec(),
            compressed.clone(),
        ];
        let server = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(10);
            let mut ranges = Vec::new();
            for body in responses {
                let mut stream = loop {
                    match listener.accept() {
                        Ok((stream, _)) => break stream,
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            assert!(Instant::now() < deadline, "timed out waiting for request");
                            thread::sleep(Duration::from_millis(10));
                        }
                        Err(e) => panic!("accept failed: {e}"),
                    }
                };
                stream.set_nonblocking(false).unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let mut reader = BufReader::new(&stream);
                loop {
                    let mut line = String::new();
                    assert_ne!(reader.read_line(&mut line).unwrap(), 0);
                    if line == "\r\n" {
                        break;
                    }
                    if line.to_ascii_lowercase().starts_with("range:") {
                        ranges.push(line.trim().to_owned());
                    }
                }
                write!(
                    stream,
                    "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )
                .unwrap();
                stream.write_all(&body).unwrap();
            }
            ranges
        });

        let mut bundle = TTBNetBundle::new(url).unwrap();
        assert_eq!(bundle.get_digest().unwrap(), bundle.get_digest().unwrap());
        let info = TTBFileInfo {
            start: 70,
            real_len: contents.len() as u32,
            gzip_len: compressed.len() as u32,
            name: "file.tex".into(),
            path: "file.tex".into(),
            hash: None,
        };
        let mut handle = bundle
            .open_fileinfo(&info, &mut NoopStatusBackend {})
            .unwrap();
        let mut actual = Vec::new();
        handle.read_to_end(&mut actual).unwrap();
        assert_eq!(actual, contents);
        let ranges = server.join().unwrap();
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].to_ascii_lowercase(), "range: bytes=0-69");
        assert_eq!(ranges[1], ranges[2]);
    }
}

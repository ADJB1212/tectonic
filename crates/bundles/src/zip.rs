// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! ZIP files as Tectonic bundles.

use crate::Bundle;
use std::{
    fs::File,
    io::{Cursor, Read, Seek},
    path::Path,
    str::FromStr,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult, digest};
use tectonic_status_base::{NoopStatusBackend, StatusBackend};
use zip::{ZipArchive, result::ZipError};

/// A bundle backed by a ZIP file.
pub struct ZipBundle<R: Read + Seek> {
    zip: ZipArchive<R>,
}

impl<R: Read + Seek> ZipBundle<R> {
    /// Create a new ZIP bundle for a generic readable and seekable stream.
    pub fn new(reader: R) -> Result<ZipBundle<R>> {
        Ok(ZipBundle {
            zip: ZipArchive::new(reader)?,
        })
    }
}

impl ZipBundle<File> {
    /// Open a file on the filesystem as a ZIP bundle.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<ZipBundle<File>> {
        Self::new(File::open(path)?)
    }
}

impl<R: Read + Seek> IoProvider for ZipBundle<R> {
    fn input_open_name(&mut self, name: &str, _status: &mut dyn StatusBackend) -> OpenResult<InputHandle> {
        // We need to be able to look at other items in the Zip file while
        // reading this one, so the only path forward is to read the entire
        // contents into a buffer right now. RAM is cheap these days.

        let mut zipitem = match self.zip.by_name(name) {
            Ok(f) => f,
            Err(e) => {
                return match e {
                    ZipError::Io(sube) => OpenResult::Err(sube.into()),
                    ZipError::FileNotFound => OpenResult::NotAvailable,
                    _ => OpenResult::Err(e.into()),
                };
            }
        };

        let s = zipitem.size();
        if s >= u32::MAX as u64 {
            return OpenResult::Err(anyhow!("Zip item too large."));
        }
        let mut buf = Vec::with_capacity(s as usize);

        if let Err(e) = zipitem.read_to_end(&mut buf) {
            return OpenResult::Err(e.into());
        }

        OpenResult::Ok(InputHandle::new_read_only(name, Cursor::new(buf), InputOrigin::Other))
    }
}

impl<R: Read + Seek> Bundle for ZipBundle<R> {
    fn all_files(&self) -> Vec<String> {
        self.zip.file_names().filter_map(|name| name.ok()).map(|name| name.into_owned()).collect()
    }

    fn get_digest(&mut self) -> Result<tectonic_io_base::digest::DigestData> {
        let digest_text = match self.input_open_name(digest::DIGEST_NAME, &mut NoopStatusBackend {}) {
            OpenResult::Ok(h) => {
                let mut text = String::new();
                h.take(64).read_to_string(&mut text)?;
                text
            }

            OpenResult::NotAvailable => {
                bail!("bundle does not provide needed SHA256SUM file");
            }

            OpenResult::Err(e) => {
                return Err(e);
            }
        };

        Ok(atry!(digest::DigestData::from_str(&digest_text); ["corrupted SHA256 digest data"]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

    #[test]
    fn lists_and_reads_bundle_files() {
        let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
        let files = [
            ("plain.tex", "plain text", CompressionMethod::Stored),
            ("caf\u{e9}.tex", "compressed text", CompressionMethod::Deflated),
            (
                digest::DIGEST_NAME,
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                CompressionMethod::Stored,
            ),
        ];

        for (name, contents, compression) in files {
            writer
                .start_file(name, SimpleFileOptions::default().compression_method(compression))
                .unwrap();
            writer.write_all(contents.as_bytes()).unwrap();
        }

        let mut bundle = ZipBundle::new(writer.finish().unwrap()).unwrap();
        let mut names = bundle.all_files();
        names.sort();
        let mut expected_names: Vec<_> = files.iter().map(|(name, _, _)| *name).collect();
        expected_names.sort();
        assert_eq!(names, expected_names);

        for (name, contents, _) in files {
            let mut handle = bundle.input_open_name(name, &mut NoopStatusBackend {}).unwrap();
            let mut text = String::new();
            handle.read_to_string(&mut text).unwrap();
            assert_eq!(text, contents);
        }

        assert_eq!(bundle.get_digest().unwrap().to_string(), files[2].1);
        assert!(matches!(
            bundle.input_open_name("missing.tex", &mut NoopStatusBackend {}),
            OpenResult::NotAvailable
        ));
    }
}

use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufReader, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};
use tectonic_engine_xetex::TexCheckpoint;
use tectonic_errors::{Result, anyhow::ensure};

const MAGIC: &[u8; 8] = b"TTINCR1\0";
const MAX_CACHE: usize = 64 * 1024 * 1024;
const MAX_STATE: usize = 512 * 1024 * 1024;

#[derive(Clone)]
pub(crate) struct Saved {
    pass: usize,
    file: Arc<Mutex<File>>,
    offset: u64,
    length: usize,
}

impl Saved {
    fn read<T>(&self, consume: impl FnOnce(&mut dyn Read) -> Result<T>) -> Result<T> {
        let mut file = self
            .file
            .lock()
            .map_err(|_| tectonic_errors::Error::msg("checkpoint file lock poisoned"))?;
        file.seek(SeekFrom::Start(self.offset))?;
        consume(&mut (&mut *file).take(self.length as u64))
    }

    pub(crate) fn decode(&self) -> Result<TexCheckpoint> {
        self.read(|reader| Cache::decode(reader))
    }
}

struct LimitedWriter<W> {
    inner: W,
    remaining: usize,
}

impl<W: Write> Write for LimitedWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let count = self.inner.write(&bytes[..bytes.len().min(self.remaining)])?;
        self.remaining -= count;
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub(crate) struct Cache {
    directory: PathBuf,
    _lock: File,
    pub(crate) build_date: SystemTime,
    saved: Vec<Saved>,
    pending: Vec<Saved>,
    pub(crate) pass: usize,
    pub(crate) captures: usize,
}

impl Cache {
    pub(crate) fn open(root: &Path, identity: &str, build_date: SystemTime) -> Result<(Self, Option<String>)> {
        let mut key = Sha256::new();
        key.update(identity.as_bytes());
        let mut executable = File::open(std::env::current_exe()?)?;
        let mut buffer = [0; 65536];
        loop {
            let n = executable.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            key.update(&buffer[..n]);
        }
        let key = tectonic_io_base::digest::bytes_to_hex(&key.finalize());
        let directory = root.join(key);
        fs::create_dir_all(&directory)?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(directory.join("lock"))?;
        lock.lock()?;
        let mut cache = Self {
            directory,
            _lock: lock,
            build_date,
            saved: Vec::new(),
            pending: Vec::new(),
            pass: 0,
            captures: 0,
        };
        let warning = match cache.load() {
            Ok(()) => None,
            Err(error) => {
                cache.saved.clear();
                Some(format!("incremental cache discarded: {error}"))
            }
        };
        Ok((cache, warning))
    }

    fn load(&mut self) -> Result<()> {
        let file = match File::open(self.directory.join("checkpoints")) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        let file_length = file.metadata()?.len();
        ensure!(file_length <= MAX_CACHE as u64, "checkpoint file is too large");
        let source = Arc::new(Mutex::new(file));
        let mut source_file = source.lock().map_err(|_| tectonic_errors::Error::msg("checkpoint file lock poisoned"))?;
        let mut file = BufReader::new((&mut *source_file).take(MAX_CACHE as u64 + 1));
        let mut header = [0; 56];
        file.read_exact(&mut header)?;
        ensure!(file_length >= 56 && &header[..8] == MAGIC, "invalid cache header");
        let mut hash = Sha256::new();
        hash.update(&header[40..]);
        let mut input = &header[40..];
        let timestamp = integer(&mut input)?;
        let count = integer(&mut input)?;
        ensure!(count <= 32, "too many checkpoints");
        let mut remaining = file_length as usize - header.len();
        let mut buffer = [0; 65536];
        let mut saved = Vec::new();
        for _ in 0..count {
            ensure!(remaining >= 16, "truncated checkpoint");
            let mut entry = [0; 16];
            file.read_exact(&mut entry)?;
            hash.update(entry);
            remaining -= entry.len();
            let mut input = entry.as_slice();
            let pass = usize::try_from(integer(&mut input)?)?;
            ensure!(pass <= 16, "invalid checkpoint pass");
            let length = usize::try_from(integer(&mut input)?)?;
            ensure!(length <= remaining, "truncated checkpoint");
            let offset = file_length - remaining as u64;
            let mut unread = length;
            while unread != 0 {
                let count = unread.min(buffer.len());
                file.read_exact(&mut buffer[..count])?;
                hash.update(&buffer[..count]);
                unread -= count;
            }
            remaining -= length;
            saved.push(Saved {
                pass,
                file: Arc::clone(&source),
                offset,
                length,
            });
        }
        ensure!(remaining == 0 && file.read(&mut [0])? == 0, "trailing checkpoint bytes");
        ensure!(hash.finalize()[..] == header[8..40], "cache checksum mismatch");
        self.saved = saved;
        self.build_date = SystemTime::UNIX_EPOCH
            .checked_add(Duration::from_secs(timestamp))
            .ok_or_else(|| tectonic_errors::Error::msg("invalid checkpoint date"))?;
        Ok(())
    }

    pub(crate) fn capture(&mut self, native: &[u8], io: &[u8]) {
        if native.len() > MAX_STATE || io.len() > MAX_CACHE {
            return;
        }
        let result = (|| -> io::Result<Saved> {
            let file = tempfile::tempfile_in(&self.directory)?;
            let writer = LimitedWriter {
                inner: file,
                remaining: MAX_CACHE / 4,
            };
            let mut encoder = ZlibEncoder::new(writer, Compression::fast());
            encoder.write_all(&(io.len() as u64).to_le_bytes())?;
            encoder.write_all(io)?;
            encoder.write_all(native)?;
            let writer = encoder.finish()?;
            Ok(Saved {
                pass: self.pass,
                file: Arc::new(Mutex::new(writer.inner)),
                offset: 0,
                length: MAX_CACHE / 4 - writer.remaining,
            })
        })();
        let Ok(saved) = result else {
            return;
        };
        if self.pending.iter().filter(|saved| saved.pass == self.pass).count() >= 2
            && let Some(index) = self.pending.iter().position(|saved| saved.pass == self.pass)
        {
            self.pending.remove(index);
        }
        self.pending.push(saved);
        trim(&mut self.pending);
        self.captures += 1;
    }

    pub(crate) fn candidates(&self) -> Vec<Saved> {
        self.saved.iter().rev().filter(|saved| saved.pass == self.pass).cloned().collect()
    }

    pub(crate) fn discard_pass(&mut self) {
        self.pending.retain(|saved| saved.pass != self.pass);
    }

    fn decode(reader: impl Read) -> Result<TexCheckpoint> {
        let mut decoder = ZlibDecoder::new(reader);
        let mut length = [0; 8];
        decoder.read_exact(&mut length)?;
        let io_length = usize::try_from(u64::from_le_bytes(length))?;
        ensure!(io_length <= MAX_CACHE, "invalid checkpoint I/O length");
        let mut io = vec![0; io_length];
        decoder.read_exact(&mut io)?;
        let mut native = Vec::new();
        decoder.take((MAX_STATE + 1) as u64).read_to_end(&mut native)?;
        ensure!(!native.is_empty() && native.len() <= MAX_STATE, "invalid native checkpoint size");
        Ok(TexCheckpoint { io, native })
    }

    pub(crate) fn commit(&mut self) -> Result<()> {
        if self.pending.is_empty() {
            return Ok(());
        }
        let passes: std::collections::HashSet<_> = self.pending.iter().map(|saved| saved.pass).collect();
        self.saved.retain(|saved| !passes.contains(&saved.pass));
        self.saved.append(&mut self.pending);
        trim(&mut self.saved);
        let timestamp = self.build_date.duration_since(SystemTime::UNIX_EPOCH)?.as_secs().to_le_bytes();
        let count = (self.saved.len() as u64).to_le_bytes();
        let mut hash = Sha256::new();
        hash.update(timestamp);
        hash.update(count);
        let mut output = tempfile::NamedTempFile::new_in(&self.directory)?;
        output.write_all(MAGIC)?;
        output.write_all(&[0; 32])?;
        output.write_all(&timestamp)?;
        output.write_all(&count)?;
        let mut buffer = [0; 65536];
        for saved in &self.saved {
            let pass = (saved.pass as u64).to_le_bytes();
            let length = (saved.length as u64).to_le_bytes();
            hash.update(pass);
            hash.update(length);
            output.write_all(&pass)?;
            output.write_all(&length)?;
            saved.read(|reader| {
                let mut remaining = saved.length;
                while remaining != 0 {
                    let count = remaining.min(buffer.len());
                    reader.read_exact(&mut buffer[..count])?;
                    hash.update(&buffer[..count]);
                    output.write_all(&buffer[..count])?;
                    remaining -= count;
                }
                Ok(())
            })?;
        }
        output.seek(SeekFrom::Start(MAGIC.len() as u64))?;
        output.write_all(&hash.finalize())?;
        output.as_file().sync_all()?;
        output.persist(self.directory.join("checkpoints"))?;
        Ok(())
    }
}

fn trim(saved: &mut Vec<Saved>) {
    while saved.len() > 24 || saved.iter().map(|saved| saved.length + 16).sum::<usize>() > MAX_CACHE - 56 {
        let index = saved
            .iter()
            .position(|entry| saved.iter().filter(|other| other.pass == entry.pass).count() > 1)
            .unwrap_or(0);
        saved.remove(index);
    }
}

fn integer(input: &mut &[u8]) -> Result<u64> {
    ensure!(input.len() >= 8, "truncated checkpoint integer");
    let (bytes, remaining) = input.split_at(8);
    *input = remaining;
    Ok(u64::from_le_bytes(bytes.try_into()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_pressure_keeps_a_recent_checkpoint_for_each_pass() {
        let frame = Arc::new(Mutex::new(tempfile::tempfile().unwrap()));
        let mut saved = Vec::new();
        for pass in 0..3 {
            for _ in 0..3 {
                saved.push(Saved {
                    pass,
                    file: Arc::clone(&frame),
                    offset: 0,
                    length: MAX_CACHE / 4 - 64,
                });
                trim(&mut saved);
            }
        }
        for pass in 0..3 {
            assert!(saved.iter().any(|entry| entry.pass == pass));
        }
        assert!(saved.iter().map(|entry| entry.length + 16).sum::<usize>() <= MAX_CACHE - 56);
    }

    #[test]
    fn checkpoint_publication_pins_time_and_detects_corruption() {
        let root = tempfile::tempdir().unwrap();
        let initial = SystemTime::UNIX_EPOCH + Duration::from_secs(1234);
        let (mut cache, warning) = Cache::open(root.path(), "test", initial).unwrap();
        assert!(warning.is_none());
        cache.capture(b"native state", b"[]");
        cache.commit().unwrap();
        let directory = cache.directory.clone();
        drop(cache);
        let (cache, warning) = Cache::open(root.path(), "test", SystemTime::now()).unwrap();
        assert!(warning.is_none());
        assert_eq!(cache.build_date, initial);
        let candidates = cache.candidates();
        let checkpoint = candidates[0].decode().unwrap();
        assert_eq!(checkpoint.native, b"native state");
        assert_eq!(checkpoint.io, b"[]");
        drop(cache);
        let mut bytes = fs::read(directory.join("checkpoints")).unwrap();
        let last = bytes.len() - 1;
        bytes[last] ^= 1;
        fs::write(directory.join("checkpoints"), bytes).unwrap();
        let (cache, warning) = Cache::open(root.path(), "test", SystemTime::now()).unwrap();
        assert!(warning.unwrap().contains("checksum mismatch"));
        assert!(cache.candidates().is_empty());
    }

    #[test]
    fn invalid_compressed_frames_are_rejected_before_native_restoration() {
        assert!(Cache::decode(b"not compressed".as_slice()).is_err());
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(&u64::MAX.to_le_bytes()).unwrap();
        assert!(Cache::decode(encoder.finish().unwrap().as_slice()).is_err());
    }

    #[test]
    fn limited_checkpoint_writes_stop_before_exceeding_the_frame_budget() {
        let mut writer = LimitedWriter {
            inner: Vec::new(),
            remaining: 3,
        };
        assert_eq!(writer.write_all(b"1234").unwrap_err().kind(), io::ErrorKind::WriteZero);
        assert_eq!(writer.inner, b"123");
        assert_eq!(writer.remaining, 0);
    }

    #[test]
    fn file_backed_candidates_survive_republication_and_concurrent_reads() {
        let root = tempfile::tempdir().unwrap();
        let (mut cache, _) = Cache::open(root.path(), "test", SystemTime::UNIX_EPOCH).unwrap();
        cache.capture(b"first", b"[]");
        cache.capture(b"second", b"[ ]");
        cache.pass = 1;
        cache.capture(b"another pass", b"[]");
        cache.commit().unwrap();
        drop(cache);

        let (mut cache, warning) = Cache::open(root.path(), "test", SystemTime::now()).unwrap();
        assert!(warning.is_none());
        let candidates = cache.candidates();
        assert_eq!(candidates[0].decode().unwrap().native, b"second");
        assert_eq!(candidates[1].decode().unwrap().native, b"first");
        cache.capture(b"replacement", b"[]");
        cache.commit().unwrap();
        drop(cache);

        std::thread::scope(|scope| {
            for (candidate, expected) in candidates.iter().zip([b"second".as_slice(), b"first".as_slice()]) {
                scope.spawn(move || {
                    for _ in 0..20 {
                        let checkpoint = candidate.decode().unwrap();
                        assert_eq!(checkpoint.native, expected);
                    }
                });
            }
        });
        let (mut cache, warning) = Cache::open(root.path(), "test", SystemTime::now()).unwrap();
        assert!(warning.is_none());
        assert_eq!(cache.candidates()[0].decode().unwrap().native, b"replacement");
        cache.pass = 1;
        assert_eq!(cache.candidates()[0].decode().unwrap().native, b"another pass");
    }

    #[test]
    fn failed_or_uncommitted_builds_leave_the_last_generation_intact() {
        let root = tempfile::tempdir().unwrap();
        let (mut cache, _) = Cache::open(root.path(), "test", SystemTime::UNIX_EPOCH).unwrap();
        cache.capture(b"successful", b"[]");
        cache.commit().unwrap();
        drop(cache);
        let (mut cache, _) = Cache::open(root.path(), "test", SystemTime::now()).unwrap();
        cache.capture(b"unsuccessful", b"[]");
        drop(cache);
        let (cache, _) = Cache::open(root.path(), "test", SystemTime::now()).unwrap();
        assert_eq!(cache.candidates()[0].decode().unwrap().native, b"successful");
    }
}

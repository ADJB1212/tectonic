use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};
use tectonic_engine_xetex::TexCheckpoint;
use tectonic_errors::{Result, anyhow::ensure};

const MAGIC: &[u8; 8] = b"TTINCR1\0";
const MAX_CACHE: usize = 64 * 1024 * 1024;
const MAX_STATE: usize = 512 * 1024 * 1024;

struct Saved {
    pass: usize,
    bytes: Arc<[u8]>,
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
    pub(crate) fn open(
        root: &Path,
        identity: &str,
        build_date: SystemTime,
    ) -> Result<(Self, Option<String>)> {
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
        ensure!(
            file.metadata()?.len() <= MAX_CACHE as u64,
            "checkpoint file is too large"
        );
        let mut bytes = Vec::new();
        file.take(MAX_CACHE as u64 + 1).read_to_end(&mut bytes)?;
        ensure!(
            bytes.len() >= 56 && bytes.len() <= MAX_CACHE && &bytes[..8] == MAGIC,
            "invalid cache header"
        );
        ensure!(
            Sha256::digest(&bytes[40..])[..] == bytes[8..40],
            "cache checksum mismatch"
        );
        let mut input = &bytes[40..];
        let timestamp = integer(&mut input)?;
        let count = integer(&mut input)?;
        ensure!(count <= 32, "too many checkpoints");
        let mut saved = Vec::new();
        for _ in 0..count {
            let pass = usize::try_from(integer(&mut input)?)?;
            ensure!(pass <= 16, "invalid checkpoint pass");
            let length = usize::try_from(integer(&mut input)?)?;
            ensure!(length <= input.len(), "truncated checkpoint");
            let (data, remaining) = input.split_at(length);
            input = remaining;
            saved.push(Saved {
                pass,
                bytes: Arc::from(data),
            });
        }
        ensure!(input.is_empty(), "trailing checkpoint bytes");
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
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
        let result = (|| -> std::io::Result<Vec<u8>> {
            encoder.write_all(&(io.len() as u64).to_le_bytes())?;
            encoder.write_all(io)?;
            encoder.write_all(native)?;
            encoder.finish()
        })();
        let Ok(bytes) = result else {
            return;
        };
        if bytes.len() > MAX_CACHE / 4 {
            return;
        }
        if self
            .pending
            .iter()
            .filter(|saved| saved.pass == self.pass)
            .count()
            >= 2
            && let Some(index) = self
                .pending
                .iter()
                .position(|saved| saved.pass == self.pass)
        {
            self.pending.remove(index);
        }
        self.pending.push(Saved {
            pass: self.pass,
            bytes: bytes.into(),
        });
        trim(&mut self.pending);
        self.captures += 1;
    }

    pub(crate) fn candidates(&self) -> Vec<Arc<[u8]>> {
        self.saved
            .iter()
            .rev()
            .filter(|saved| saved.pass == self.pass)
            .map(|saved| Arc::clone(&saved.bytes))
            .collect()
    }

    pub(crate) fn discard_pass(&mut self) {
        self.pending.retain(|saved| saved.pass != self.pass);
    }

    pub(crate) fn decode(bytes: &[u8]) -> Result<TexCheckpoint> {
        let mut decoder = ZlibDecoder::new(bytes);
        let mut length = [0; 8];
        decoder.read_exact(&mut length)?;
        let io_length = usize::try_from(u64::from_le_bytes(length))?;
        ensure!(io_length <= MAX_CACHE, "invalid checkpoint I/O length");
        let mut io = vec![0; io_length];
        decoder.read_exact(&mut io)?;
        let mut native = Vec::new();
        decoder
            .take((MAX_STATE + 1) as u64)
            .read_to_end(&mut native)?;
        ensure!(
            !native.is_empty() && native.len() <= MAX_STATE,
            "invalid native checkpoint size"
        );
        Ok(TexCheckpoint { io, native })
    }

    pub(crate) fn commit(&mut self) -> Result<()> {
        if self.pending.is_empty() {
            return Ok(());
        }
        let passes: std::collections::HashSet<_> =
            self.pending.iter().map(|saved| saved.pass).collect();
        self.saved.retain(|saved| !passes.contains(&saved.pass));
        self.saved.append(&mut self.pending);
        trim(&mut self.saved);
        let timestamp = self
            .build_date
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            .to_le_bytes();
        let count = (self.saved.len() as u64).to_le_bytes();
        let mut hash = Sha256::new();
        hash.update(timestamp);
        hash.update(count);
        for saved in &self.saved {
            hash.update((saved.pass as u64).to_le_bytes());
            hash.update((saved.bytes.len() as u64).to_le_bytes());
            hash.update(&saved.bytes);
        }
        let mut output = tempfile::NamedTempFile::new_in(&self.directory)?;
        output.write_all(MAGIC)?;
        output.write_all(&hash.finalize())?;
        output.write_all(&timestamp)?;
        output.write_all(&count)?;
        for saved in &self.saved {
            output.write_all(&(saved.pass as u64).to_le_bytes())?;
            output.write_all(&(saved.bytes.len() as u64).to_le_bytes())?;
            output.write_all(&saved.bytes)?;
        }
        output.as_file().sync_all()?;
        output.persist(self.directory.join("checkpoints"))?;
        Ok(())
    }
}

fn trim(saved: &mut Vec<Saved>) {
    while saved.len() > 24
        || saved
            .iter()
            .map(|saved| saved.bytes.len() + 16)
            .sum::<usize>()
            > MAX_CACHE - 56
    {
        let index = saved
            .iter()
            .position(|entry| {
                saved
                    .iter()
                    .filter(|other| other.pass == entry.pass)
                    .count()
                    > 1
            })
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
        let frame: Arc<[u8]> = vec![0; MAX_CACHE / 4 - 64].into();
        let mut saved = Vec::new();
        for pass in 0..3 {
            for _ in 0..3 {
                saved.push(Saved {
                    pass,
                    bytes: Arc::clone(&frame),
                });
                trim(&mut saved);
            }
        }
        for pass in 0..3 {
            assert!(saved.iter().any(|entry| entry.pass == pass));
        }
        assert!(
            saved
                .iter()
                .map(|entry| entry.bytes.len() + 16)
                .sum::<usize>()
                <= MAX_CACHE - 56
        );
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
        let checkpoint = Cache::decode(&candidates[0]).unwrap();
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
        assert!(Cache::decode(b"not compressed").is_err());
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(&u64::MAX.to_le_bytes()).unwrap();
        assert!(Cache::decode(&encoder.finish().unwrap()).is_err());
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
        assert_eq!(
            Cache::decode(&cache.candidates()[0]).unwrap().native,
            b"successful"
        );
    }
}

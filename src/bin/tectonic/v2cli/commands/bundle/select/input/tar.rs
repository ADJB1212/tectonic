use anyhow::Result;
use std::{
    fs::File,
    io::{Read, Seek},
    path::{Path, PathBuf},
};
use tar::Archive;
use tectonic_io_base::OutputHandle;
use tracing::info;

use super::BundleInput;

pub struct TarBundleInput {
    archive: Archive<File>,
    root: PathBuf,
    hash: String,
}

impl TarBundleInput {
    pub fn new(path: impl AsRef<Path>, root: Option<PathBuf>) -> Result<Self> {
        let path = path.as_ref().canonicalize()?;
        let mut file = File::open(&path)?;

        info!("computing hash of {}", path.to_str().unwrap());

        let hash = {
            let mut hasher = OutputHandle::new("", std::io::sink());
            let _ = std::io::copy(&mut file, &mut hasher)?;
            hasher.into_name_digest().1.to_string()
        };

        file.seek(std::io::SeekFrom::Start(0))?;
        Ok(Self {
            archive: Archive::new(file),
            root: root.unwrap_or_default(),
            hash,
        })
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }
}

impl BundleInput for TarBundleInput {
    fn iter_files(&mut self) -> impl Iterator<Item = Result<(String, Box<dyn Read + '_>)>> {
        let root = self.root.clone();
        self.archive.entries().unwrap().filter_map(move |x| {
            // TODO: error handling
            let xr = x.as_ref().unwrap();

            if !xr.header().entry_type().is_file() {
                None
            } else {
                let path = xr.path().unwrap();

                if !path.starts_with(&root) {
                    None
                } else {
                    Some(Ok((
                        path.strip_prefix(&root)
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                        Box::new(x.unwrap()) as Box<dyn Read>,
                    )))
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::fs;
    use tectonic_io_base::digest::bytes_to_hex;

    #[test]
    fn archive_hash_matches_bytes_and_leaves_entries_readable() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("input.tar");
        let contents = b"bundle file\n";
        let mut archive = tar::Builder::new(File::create(&path).unwrap());
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive
            .append_data(&mut header, "root/file.txt", &contents[..])
            .unwrap();
        archive.finish().unwrap();
        drop(archive);

        let mut input = TarBundleInput::new(&path, Some(PathBuf::from("root"))).unwrap();
        assert_eq!(
            input.hash(),
            bytes_to_hex(&Sha256::digest(fs::read(&path).unwrap()))
        );
        let mut entries = input.iter_files();
        let (name, mut reader) = entries.next().unwrap().unwrap();
        assert_eq!(name, "file.txt");
        let mut actual = Vec::new();
        reader.read_to_end(&mut actual).unwrap();
        assert_eq!(actual, contents);
        drop(reader);
        assert!(entries.next().is_none());
    }
}

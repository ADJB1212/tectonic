use super::BundleInput;
use anyhow::Result;
use std::{
    fs::{self},
    io::Read,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub struct DirBundleInput {
    dir: PathBuf,
}

impl DirBundleInput {
    pub fn new(dir: impl AsRef<Path>) -> Self {
        Self {
            dir: dir.as_ref().canonicalize().unwrap(),
        }
    }
}

impl BundleInput for DirBundleInput {
    fn iter_files(&mut self) -> impl Iterator<Item = Result<(String, Box<dyn Read + '_>)>> {
        WalkDir::new(&self.dir)
            .into_iter()
            .filter_map(|x| match x {
                Err(_) => Some(x),
                Ok(x) => {
                    if !x.file_type().is_file() {
                        None
                    } else {
                        Some(Ok(x))
                    }
                }
            })
            .map(move |x| match x {
                Ok(entry) => {
                    let path_buf = entry.into_path();
                    let rel_path = path_buf.strip_prefix(&self.dir).unwrap().to_str().unwrap().to_string();

                    let file = fs::File::open(&path_buf)?;
                    Ok((rel_path, Box::new(file) as Box<dyn Read>))
                }
                Err(e) => Err(anyhow::Error::from(e)),
            })
    }
}

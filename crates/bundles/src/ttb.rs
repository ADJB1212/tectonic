// Copyright 2023-2024 the Tectonic Project
// Licensed under the MIT License.

//! Common tools for the ttbv1 format, used in both
//! network and filesystem bundles.

use crate::{FileIndex, FileInfo};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    io::{BufRead, BufReader, Read},
    str::FromStr,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::digest::{self, DigestData};

#[derive(Clone, Copy)]
pub struct TTBv1Header {
    pub index_start: u64,
    pub index_real_len: u32,
    pub index_gzip_len: u32,
    pub digest: DigestData,
}

impl TryFrom<[u8; 70]> for TTBv1Header {
    type Error = Error;

    fn try_from(header: [u8; 70]) -> Result<Self, Self::Error> {
        let signature = &header[0..14];
        let version = u32::from_le_bytes(header[14..18].try_into()?);
        let index_start = u64::from_le_bytes(header[18..26].try_into()?);
        let index_gzip_len = u32::from_le_bytes(header[26..30].try_into()?);
        let index_real_len = u32::from_le_bytes(header[30..34].try_into()?);
        let digest: DigestData = DigestData::from_str(&digest::bytes_to_hex(&header[34..66]))?;

        if signature != b"tectonicbundle" {
            bail!("this is not a bundle");
        }

        if version != 1 {
            bail!("wrong ttb version");
        }

        Ok(TTBv1Header {
            digest,
            index_start,
            index_real_len,
            index_gzip_len,
        })
    }
}

/// file info for TTbundle
#[derive(Clone, Debug)]
pub struct TTBFileInfo {
    pub start: u64,
    pub real_len: u32,
    pub gzip_len: u32,
    pub path: String,
    pub name: String,
    pub hash: Option<String>,
}

impl FileInfo for TTBFileInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Default, Debug)]
pub struct TTBFileIndex {
    // Vector of fileinfos.
    pub content: Vec<TTBFileInfo>,

    search_orders: HashMap<String, Vec<String>>,
    default_search_order: String,

    // Remember previous searches so we don't have to iterate over content again.
    search_cache: HashMap<String, Option<usize>>,
    by_name: HashMap<String, Vec<usize>>,
}

impl TTBFileIndex {
    fn ensure_by_name(&mut self) {
        if self.by_name.is_empty() && !self.content.is_empty() {
            for (idx, info) in self.content.iter().enumerate() {
                self.by_name.entry(info.name.clone()).or_default().push(idx);
            }
        }
    }

    fn read_filelist_line(&mut self, line: &str) -> Result<()> {
        let mut bits = line.split_whitespace();

        if let (Some(start), Some(gzip_len), Some(real_len), Some(hash)) = (bits.next(), bits.next(), bits.next(), bits.next()) {
            let path = match bits.next() {
                Some(first) => match bits.next() {
                    None => first.to_owned(),
                    Some(second) => {
                        let mut p = String::with_capacity(first.len() + 1 + second.len());
                        p.push_str(first);
                        p.push(' ');
                        p.push_str(second);
                        for rest in bits {
                            p.push(' ');
                            p.push_str(rest);
                        }
                        p
                    }
                },
                None => bail!("malformed FILELIST line: missing path"),
            };
            let name = path.rsplit_once('/').map(|(_, n)| n).unwrap_or(&path).to_owned();

            // Basic path validation.
            // TODO: more robust checks
            if path.starts_with('/')
                || path.contains("./") // Also catches "/../"
                || path.contains("//")
            {
                bail!("bad bundle file path `{path}`");
            }

            let idx = self.content.len();
            self.by_name.entry(name.clone()).or_default().push(idx);

            self.content.push(TTBFileInfo {
                start: start.parse::<u64>()?,
                gzip_len: gzip_len.parse::<u32>()?,
                real_len: real_len.parse::<u32>()?,
                path,
                name,
                hash: match hash {
                    "nohash" => None,
                    _ => Some(hash.to_owned()),
                },
            });
        } else {
            // TODO: preserve the warning info or something!
            bail!("malformed FILELIST line");
        }

        Ok(())
    }

    fn read_search_line(&mut self, name: String, line: String) -> Result<()> {
        let stat = self.search_orders.entry(name).or_default();
        stat.push(line);
        Ok(())
    }

    fn read_defaultsearch_line(&mut self, line: String) -> Result<()> {
        self.default_search_order = line;
        Ok(())
    }
}

impl<'this> FileIndex<'this> for TTBFileIndex {
    type InfoType = TTBFileInfo;

    fn iter(&'this self) -> Box<dyn Iterator<Item = &'this TTBFileInfo> + 'this> {
        Box::new(self.content.iter())
    }

    fn len(&self) -> usize {
        self.content.len()
    }

    fn initialize(&mut self, reader: &mut dyn Read) -> Result<()> {
        self.content.clear();
        self.search_orders.clear();
        self.search_cache.clear();
        self.default_search_order.clear();
        self.by_name.clear();

        let mut mode: String = String::new();
        for line in BufReader::new(reader).lines() {
            let line = line?;

            if line.starts_with('[') {
                line[1..line.len() - 1].clone_into(&mut mode);
                continue;
            }

            if mode.is_empty() {
                continue;
            }

            let (cmd, arg) = mode.rsplit_once(':').unwrap_or((&mode[..], ""));

            match cmd {
                "DEFAULTSEARCH" => self.read_defaultsearch_line(line)?,
                "FILELIST" => self.read_filelist_line(&line)?,
                "SEARCH" => self.read_search_line(arg.to_owned(), line)?,
                _ => continue,
            }
        }

        Ok(())
    }

    fn search(&'this mut self, name: &str) -> Option<TTBFileInfo> {
        match self.search_cache.get(name) {
            None => {}
            Some(r) => return r.map(|idx| self.content[idx].clone()),
        }

        self.ensure_by_name();

        let search = match self.search_orders.get(&self.default_search_order) {
            Some(s) => s,
            None => {
                self.search_cache.insert(name.to_owned(), None);
                return None;
            }
        };

        // Edge case: absolute paths
        if name.starts_with('/') {
            return None;
        }

        // Get last element of path, since
        // some packages reference a path to a file.
        // `fithesis4` is one example.
        let relative_parent: bool;

        let n = match name.rsplit_once('/') {
            Some(n) => {
                relative_parent = true;
                n.1
            }
            None => {
                relative_parent = false;
                name
            }
        };

        let matching_indices = match self.by_name.get(n) {
            Some(indices) => indices,
            None => {
                self.search_cache.insert(name.to_owned(), None);
                return None;
            }
        };

        if relative_parent {
            // TODO: REWORK
            let mut matching = None;
            for &idx in matching_indices {
                let info = &self.content[idx];
                if info.path().ends_with(name) {
                    match matching {
                        Some(_) => {
                            self.search_cache.insert(name.to_owned(), None);
                            return None;
                        }
                        None => matching = Some(idx),
                    }
                }
            }
            self.search_cache.insert(name.to_owned(), matching);
            matching.map(|idx| self.content[idx].clone())
        } else {
            // Even if paths.len() is 1, we don't return here.
            // We need to make sure this file matches a search path:
            // if it's in a directory we don't search, we shouldn't find it!

            let mut picked: Option<usize> = None;
            for rule in search {
                // Remove leading slash from rule
                // (search patterns start with slashes, but paths do not)
                let rule = &rule[1..];

                for &idx in matching_indices {
                    let info = &self.content[idx];
                    let matches = if rule.ends_with("//") {
                        // Match start of parent path
                        // (cutting off the last slash)
                        info.path().starts_with(&rule[0..rule.len() - 1])
                    } else {
                        // Match full parent path
                        let prefix_len = info.path().len().saturating_sub(name.len());
                        &info.path()[0..prefix_len] == rule
                    };
                    if matches && picked.is_none_or(|previous| info.path < self.content[previous].path) {
                        picked = Some(idx);
                    }
                }
                if picked.is_some() {
                    break;
                }
            }

            self.search_cache.insert(name.to_owned(), picked);
            picked.map(|idx| self.content[idx].clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn index() -> TTBFileIndex {
        let mut index = TTBFileIndex::default();
        index
            .initialize(&mut Cursor::new(
                b"[DEFAULTSEARCH]\nmain\n[SEARCH:main]\n/first//\n/second//\n[FILELIST]\n\
                  0 0 0 nohash second/a/file.tex\n\
                  0 0 0 nohash first/z/file.tex\n\
                  0 0 0 nohash first/a/file.tex\n\
                  0 0 0 nohash hidden/hidden.tex\n\
                  0 0 0 nohash first/exact.tex\n",
            ))
            .unwrap();
        index
    }

    #[test]
    fn search_preserves_rule_priority_and_alphabetical_order() {
        let mut index = index();
        for _ in 0..2 {
            assert_eq!(index.search("file.tex").unwrap().path, "first/a/file.tex");
            assert!(index.search("hidden.tex").is_none());
            assert!(index.search("missing.tex").is_none());
            assert_eq!(index.search("z/file.tex").unwrap().path, "first/z/file.tex");
            assert!(index.search("a/file.tex").is_none());
            assert!(index.search("/first/exact.tex").is_none());
        }
    }

    #[test]
    fn exact_search_does_not_match_subdirectories() {
        let mut index = index();
        index.search_orders.insert("main".into(), vec!["/first/".into()]);
        assert!(index.search("file.tex").is_none());
        assert_eq!(index.search("exact.tex").unwrap().path, "first/exact.tex");
    }

    #[test]
    fn reinitializing_clears_cached_hits_and_misses() {
        let mut index = index();
        assert!(index.search("file.tex").is_some());
        assert!(index.search("new.tex").is_none());
        index
            .initialize(&mut Cursor::new(
                b"[DEFAULTSEARCH]\nmain\n[SEARCH:main]\n/new/\n[FILELIST]\n0 0 0 nohash new/new.tex\n",
            ))
            .unwrap();
        assert!(index.search("file.tex").is_none());
        assert_eq!(index.search("new.tex").unwrap().path, "new/new.tex");
    }
}

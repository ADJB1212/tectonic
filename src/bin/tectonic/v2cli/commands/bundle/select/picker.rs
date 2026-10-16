use anyhow::{Context, Result, bail};
use digest_io::{HashReader, IoWrapper};
use regex::RegexSet;
use sha2::{Digest, Sha256};
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
    io::{self, BufWriter, Cursor, Read, Write},
    iter::FromIterator,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use tectonic_io_base::digest::bytes_to_hex;
use tracing::{debug, error, info, trace, warn};
use walkdir::WalkDir;

use crate::v2cli::commands::bundle::create::BundleCreateCommand;

use super::{
    input::Input,
    spec::BundleSearchOrder,
    spec::{BundleInputSource, BundleSpec},
};

#[derive(Default)]
pub struct PickStatistics {
    /// Total number of files added from each source
    added: HashMap<String, usize>,

    /// Number of file conflicts
    conflicts: usize,

    /// Total number of files ignored
    ignored: usize,

    /// Total number of patches applied
    patch_applied: usize,

    /// Total number of patches found
    patch_found: usize,
}

impl PickStatistics {
    /// Returns a pretty status summary string
    pub fn make_string(&self) -> String {
        let mut output_string = format!(
            concat!(
                "=============== Summary ===============\n",
                "    file conflicts:       {}\n",
                "    files ignored:        {}\n",
                "    diffs applied/found:  {}/{}\n",
                "    =============================\n",
            ),
            self.conflicts, self.ignored, self.patch_applied, self.patch_found,
        );

        let mut sum = 0;
        for (source, count) in &self.added {
            let s = format!("{source} files: ");
            output_string.push_str(&format!("    {s}{}{count}\n", " ".repeat(22 - s.len())));
            sum += count;
        }
        output_string.push_str(&format!("    total files:          {sum}\n\n"));

        output_string.push_str(&"=".repeat(39));
        output_string
    }

    /// Did we find as many, fewer, or more patches than we applied?
    pub fn compare_patch_found_applied(&self) -> Ordering {
        self.patch_found.cmp(&self.patch_applied)
    }
}

struct FileListEntry {
    /// Path relative to content dir (does not start with a slash)
    path: PathBuf,
    hash: Option<String>,
}

impl Display for FileListEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            match &self.hash {
                Some(s) => &s,
                None => "nohash",
            },
            self.path.to_str().unwrap(),
        )
    }
}

pub struct FilePicker {
    /// This bundle specification's root directory.
    /// (i.e, where we found bundle.toml)
    bundle_dir: PathBuf,

    /// Where to place this bundle's files
    build_dir: PathBuf,

    /// This file picker's statistics
    pub stats: PickStatistics,

    /// All files we've picked so far.
    /// This map's keys are the `path` value of `FileListEntry`.
    filelist: HashMap<PathBuf, FileListEntry>,

    bundle_spec: BundleSpec,
}

impl FilePicker {
    /// Transform a search order file with shortcuts
    /// (bash-like brace expansion, like `/a/b/{tex,latex}/c`)
    /// into a plain list of strings.
    fn expand_search_line(s: &str) -> Result<Vec<String>> {
        if !(s.contains('{') || s.contains('}')) {
            return Ok(vec![s.to_owned()]);
        }

        let first = match s.find('{') {
            Some(x) => x,
            None => bail!("Bad search path format"),
        };

        let last = match s.find('}') {
            Some(x) => x,
            None => bail!("Bad search path format"),
        };

        let head = &s[..first];
        let mid = &s[first + 1..last];

        if mid.contains('{') || mid.contains('}') {
            // Mismatched or nested braces
            bail!("Bad search path format");
        }

        // We find the first brace, so only tail may have other expansions.
        let tail = Self::expand_search_line(&s[last + 1..s.len()])?;

        if mid.is_empty() {
            bail!("Bad search path format");
        }

        let mut output: Vec<String> = Vec::new();
        for m in mid.split(',') {
            for t in &tail {
                if m.is_empty() {
                    bail!("Bad search path format");
                }
                output.push(format!("{head}{m}{t}"));
            }
        }

        Ok(output)
    }

    /// Patch a file in-place.
    /// This should be done after calling `add_file`.
    fn apply_patch(
        &mut self,
        path: &Path,
        path_in_source: &Path,
        diffs: &HashMap<PathBuf, PathBuf>,
    ) -> Result<bool> {
        // Is this file patched?
        if !diffs.contains_key(path_in_source) {
            return Ok(false);
        }

        info!("patching `{}`", path_in_source.to_str().unwrap());

        self.stats.patch_applied += 1;

        // Discard first line of diff
        let diff_file = fs::read_to_string(&diffs[path_in_source]).unwrap();
        let (_, diff) = diff_file.split_once('\n').unwrap();

        // TODO: don't require `patch`
        let mut child = Command::new("patch")
            .arg("--quiet")
            .arg("--no-backup")
            .arg(path)
            .stdin(Stdio::piped())
            .spawn()
            .context("while spawning `patch`")?;

        let mut stdin = child.stdin.take().unwrap();
        stdin
            .write_all(diff.as_bytes())
            .context("while passing diff to `patch`")?;
        drop(stdin);
        child.wait().context("while waiting for `patch`")?;

        Ok(true)
    }

    /// Add a file into the file list.
    fn add_to_filelist(&mut self, path: PathBuf, hash: Option<String>) {
        trace!("adding `{path:?}` to file list");

        self.filelist
            .insert(path.clone(), FileListEntry { hash, path });
    }

    fn hash_file(path: &Path) -> Result<String> {
        let mut hasher = IoWrapper(Sha256::new());
        io::copy(
            &mut File::open(path).with_context(|| format!("while computing hash of {path:?}"))?,
            &mut hasher,
        )?;
        Ok(bytes_to_hex(&hasher.0.finalize()))
    }

    /// Add a file to this picker's content directory
    fn add_file(
        &mut self,
        path_in_source: &Path,
        source: &str,
        file_content: &mut dyn Read,
        diffs: &HashMap<PathBuf, PathBuf>,
    ) -> Result<()> {
        let target_path = self
            .build_dir
            .join("content")
            .join(source)
            .join(path_in_source);

        // Path to this file, relative to content dir
        let rel = target_path
            .strip_prefix(self.build_dir.join("content"))
            .unwrap()
            .to_path_buf();

        trace!("adding {path_in_source:?} from source `{source}`");

        // Skip files that already exist
        if self.filelist.contains_key(&rel) {
            self.stats.conflicts += 1;
            warn!("{path_in_source:?} from source `{source}` already exists, skipping");
            return Ok(());
        }

        fs::create_dir_all(match target_path.parent() {
            Some(x) => x,
            None => bail!("couldn't get parent of target"),
        })
        .context("failed to create content directory")?;

        // Copy to content dir.
        let mut file = fs::File::create(&target_path)?;
        let mut hashed_content = HashReader::<Sha256, _>::new(file_content);
        io::copy(&mut hashed_content, &mut file).with_context(|| {
            format!("while writing file `{path_in_source:?}` from source `{source}`")
        })?;
        drop(file);

        // Apply patch if one exists
        let patched = self
            .apply_patch(&target_path, path_in_source, diffs)
            .with_context(|| {
                format!("while patching `{path_in_source:?}` from source `{source}`")
            })?;

        let hash = if patched {
            Self::hash_file(&target_path)?
        } else {
            bytes_to_hex(&hashed_content.finalize())
        };
        self.add_to_filelist(rel, Some(hash));

        Ok(())
    }
}

// Public methods
impl FilePicker {
    /// Create a new file picker working in build_dir
    pub fn new(bundle_spec: BundleSpec, build_dir: PathBuf, bundle_dir: PathBuf) -> Result<Self> {
        if !build_dir.is_dir() {
            bail!("build_dir is not a directory!")
        }

        if build_dir.read_dir()?.next().is_some() {
            bail!("build_dir is not empty!")
        }

        Ok(FilePicker {
            bundle_dir,
            build_dir,
            filelist: HashMap::new(),
            bundle_spec,
            stats: PickStatistics::default(),
        })
    }

    /// Iterate over this bundle's sources
    pub fn iter_sources(&self) -> impl Iterator<Item = &String> {
        self.bundle_spec.inputs.keys()
    }

    /// Add a directory of files to this bundle under `source_name`,
    /// applying patches and checking for replacements.
    pub fn add_source(&mut self, cli: &BundleCreateCommand, source: &str) -> Result<()> {
        info!("adding source `{source}`");

        let input = self.bundle_spec.inputs.get(source).unwrap().clone();
        let mut added = 0usize;

        // Load diff files
        let diffs = input
            .patch_dir
            .as_ref()
            .map(|x| -> Result<HashMap<PathBuf, PathBuf>> {
                let mut diffs = HashMap::new();

                for entry in WalkDir::new(self.bundle_dir.join(x)) {
                    // Only iterate files
                    let entry = entry?;
                    if !entry.file_type().is_file() {
                        continue;
                    }
                    let entry = entry.into_path();

                    // Only include files with a `.diff extension`
                    if entry.extension().map(|x| x != "diff").unwrap_or(true) {
                        continue;
                    }

                    // Read first line of diff to get target path
                    let diff_file = fs::read_to_string(&entry).unwrap();
                    let (target, _) = diff_file.split_once('\n').unwrap();

                    trace!(tectonic_log_source = "select", "adding diff {entry:?}");

                    for t in Self::expand_search_line(target)?
                        .into_iter()
                        .map(PathBuf::from)
                    {
                        if diffs.contains_key(&t) {
                            warn!("the target of diff {entry:?} conflicts with another, ignoring");
                            continue;
                        }

                        diffs.insert(t, entry.clone());
                        self.stats.patch_found += 1;
                    }
                }

                Ok(diffs)
            })
            .unwrap_or(Ok(HashMap::new()))?;

        // Load and compile ignore patterns
        let ignore_patterns = {
            let mut patterns = Vec::new();
            if let Some(ignore) = &self.bundle_spec.bundle.ignore {
                for x in ignore {
                    patterns.push(format!("^{x}$"));
                }
            }
            if let Some(ignore) = &input.ignore {
                for x in ignore {
                    patterns.push(format!("^/{source}/{x}$"));
                }
            }
            RegexSet::new(patterns)?
        };

        let mut source_backend = match &input.source {
            BundleInputSource::Directory { path, .. } => Input::new_dir(self.bundle_dir.join(path)),
            BundleInputSource::Tarball {
                path,
                root_dir,
                hash,
            } => {
                let x = match Input::new_tarball(self.bundle_dir.join(path), root_dir.clone()) {
                    Ok(x) => x,
                    Err(e) => {
                        error!("could not add source `{source}` from tarball");
                        return Err(e);
                    }
                };
                let hash = hash.clone();
                self.add_file(
                    Path::new("TAR-SHA256SUM"),
                    source,
                    &mut Cursor::new(format!("{}\n", x.hash().unwrap())),
                    &HashMap::new(),
                )?;

                if x.hash().unwrap() != hash {
                    if cli.allow_hash_mismatch {
                        warn!("hash of tarball for source `{source}` doesn't match expected value");
                        warn!("expected: {}", hash);
                        warn!("got:      {}", x.hash().unwrap());
                    } else {
                        error!(
                            "hash of tarball for source `{source}` doesn't match expected value"
                        );
                        error!("expected: {}", hash);
                        error!("got:      {}", x.hash().unwrap());
                        bail!("hash of tarball for source `{source}` doesn't match expected value")
                    }
                }

                info!("OK, tar hash matches bundle config");
                x
            }
        };

        for x in source_backend.iter_files() {
            let (rel_file_path, mut read) = x?;

            let ignore = if ignore_patterns.is_empty() {
                false
            } else {
                let f = format!("/{source}/{rel_file_path}");
                ignore_patterns.is_match(&f)
            };

            // Skip ignored files
            if ignore {
                debug!(
                    "skipping file {rel_file_path:?} from source `{source}` because of ignore patterns"
                );
                self.stats.ignored += 1;
                continue;
            }

            // Debug info
            if self.filelist.len() % 1937 == 1936 {
                info!("selecting files ({source}, {})", self.filelist.len());
            }

            trace!("adding file {rel_file_path:?} from source `{source}`");

            self.add_file(Path::new(&rel_file_path), source, &mut read, &diffs)
                .with_context(|| format!("while adding file `{rel_file_path:?}`"))?;
            added += 1;
        }

        self.stats.added.insert(source.to_owned(), added);

        Ok(())
    }

    pub fn finish(&mut self, save_debug_files: bool) -> Result<()> {
        info!("writing auxillary files");

        // Save search specification
        let search = {
            let mut search = Vec::new();
            let path = self.build_dir.join("content/SEARCH");

            for s in &self.bundle_spec.bundle.search_order {
                match s {
                    BundleSearchOrder::Plain(s) => {
                        for i in Self::expand_search_line(s)? {
                            search.push(i);
                        }
                    }
                    BundleSearchOrder::Input { input } => {
                        let s = &self.bundle_spec.inputs.get(input).unwrap().search_order;
                        if let Some(s) = s {
                            for line in s {
                                for i in Self::expand_search_line(&format!("/{input}/{line}"))? {
                                    search.push(i);
                                }
                            }
                        } else {
                            for i in Self::expand_search_line(&format!("/{input}//"))? {
                                search.push(i);
                            }
                        }
                    }
                }
            }

            let mut file = BufWriter::new(File::create(&path).context("while writing SEARCH")?);
            for s in &search {
                writeln!(file, "{s}")?;
            }
            file.flush()?;

            let hash = Self::hash_file(&path)?;
            self.add_to_filelist(PathBuf::from("SEARCH"), Some(hash));

            search
        };

        {
            // These aren't hashed, but must be listed anyway.
            // The hash is generated from the filelist, so we must add these before hashing.
            self.add_to_filelist(PathBuf::from("SHA256SUM"), None);
            self.add_to_filelist(PathBuf::from("FILELIST"), None);

            let mut filelist_vec = Vec::from_iter(self.filelist.values());
            filelist_vec.sort_by(|a, b| a.path.cmp(&b.path));

            let filelist_path = self.build_dir.join("content/FILELIST");

            // Save FILELIST.
            let mut file =
                BufWriter::new(File::create(&filelist_path).context("while writing FILELIST")?);
            for entry in filelist_vec {
                writeln!(file, "{entry}")?;
            }
            file.flush()?;

            // Compute and save hash
            let mut file = File::create(self.build_dir.join("content/SHA256SUM"))
                .context("while writing SHA256SUM")?;

            let mut hasher = IoWrapper(Sha256::new());
            let _ = std::io::copy(&mut fs::File::open(&filelist_path)?, &mut hasher)?;
            let hash = bytes_to_hex(&hasher.0.finalize());

            writeln!(file, "{hash}")?;
        }

        if save_debug_files {
            // Generate search-report
            {
                let mut file = BufWriter::new(
                    File::create(self.build_dir.join("search-report"))
                        .context("while writing search-report")?,
                );
                for entry in WalkDir::new(self.build_dir.join("content")) {
                    let entry = entry?;
                    if !entry.file_type().is_dir() {
                        continue;
                    }
                    let entry = entry
                        .into_path()
                        .strip_prefix(self.build_dir.join("content"))
                        .unwrap()
                        .to_owned();
                    let entry = PathBuf::from("/").join(entry);

                    // Will this directory be searched?
                    let mut is_searched = false;
                    for rule in &search {
                        if rule.ends_with("//") {
                            // Match start of patent path
                            // (cutting off the last slash from)
                            if entry.starts_with(&rule[0..rule.len() - 1]) {
                                is_searched = true;
                                break;
                            }
                        } else {
                            // Match full parent path
                            if entry.to_str().unwrap() == rule {
                                is_searched = true;
                                break;
                            }
                        }
                    }

                    if !is_searched {
                        let s = entry.to_str().unwrap();
                        let t = s.matches('/').count();
                        writeln!(file, "{}{s}", "\t".repeat(t - 1))?;
                    }
                }
                file.flush()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::spec::BundleConfig;
    use super::*;

    fn picker(build_dir: &Path) -> FilePicker {
        let spec = BundleSpec {
            bundle: BundleConfig {
                name: "test".into(),
                expected_hash: String::new(),
                search_order: Vec::new(),
                ignore: None,
            },
            inputs: HashMap::new(),
        };
        FilePicker::new(spec, build_dir.to_owned(), build_dir.to_owned()).unwrap()
    }

    #[test]
    fn copied_file_hashes_match_contents_and_conflicts_preserve_original() {
        let dir = tempfile::tempdir().unwrap();
        let mut picker = picker(dir.path());
        for (name, contents) in [("empty", &[][..]), ("binary", &[0, 255, 1, 2][..])] {
            picker
                .add_file(
                    Path::new(name),
                    "source",
                    &mut Cursor::new(contents),
                    &HashMap::new(),
                )
                .unwrap();
            let relative = PathBuf::from("source").join(name);
            assert_eq!(
                picker.filelist[&relative].hash.as_deref(),
                Some(bytes_to_hex(&Sha256::digest(contents)).as_str())
            );
            picker
                .add_file(
                    Path::new(name),
                    "source",
                    &mut Cursor::new(b"replacement"),
                    &HashMap::new(),
                )
                .unwrap();
            assert_eq!(
                fs::read(dir.path().join("content").join(relative)).unwrap(),
                contents
            );
        }
        assert_eq!(picker.stats.conflicts, 2);
    }

    #[cfg(unix)]
    #[test]
    fn patched_file_hash_uses_modified_contents() {
        let dir = tempfile::tempdir().unwrap();
        let mut picker = picker(dir.path());
        let patch = dir.path().join("change.diff");
        fs::write(
            &patch,
            "file.txt\n--- file.txt\n+++ file.txt\n@@ -1 +1 @@\n-before\n+after\n",
        )
        .unwrap();
        let diffs = HashMap::from([(PathBuf::from("file.txt"), patch)]);
        picker
            .add_file(
                Path::new("file.txt"),
                "source",
                &mut Cursor::new(b"before\n"),
                &diffs,
            )
            .unwrap();
        assert_eq!(
            fs::read(dir.path().join("content/source/file.txt")).unwrap(),
            b"after\n"
        );
        assert_eq!(
            picker.filelist[Path::new("source/file.txt")]
                .hash
                .as_deref(),
            Some(bytes_to_hex(&Sha256::digest(b"after\n")).as_str())
        );
        assert_eq!(picker.stats.patch_applied, 1);
    }
}

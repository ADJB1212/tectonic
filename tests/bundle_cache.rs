use std::{
    cell::Cell,
    fs,
    io::{Cursor, Read},
    rc::Rc,
};
use tectonic_bundles::{
    Bundle, CachableBundle, FileIndex, FileInfo,
    cache::BundleCache,
    itar::{ItarFileIndex, ItarFileInfo},
};
use tectonic_errors::Result;
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult, digest::DigestData};
use tectonic_status_base::{NoopStatusBackend, StatusBackend};

const INDEX: &[u8] = b"file.tex 0 8\nnested/missing.tex 8 8\n";

#[derive(Default)]
struct TestBundle {
    index: ItarFileIndex,
    requests: Rc<Cell<usize>>,
}

impl TestBundle {
    fn requested(&self) {
        self.requests.set(self.requests.get() + 1);
    }
}

impl IoProvider for TestBundle {}

impl Bundle for TestBundle {
    fn get_digest(&mut self) -> Result<DigestData> {
        self.requested();
        Ok(DigestData::zeros())
    }

    fn all_files(&self) -> Vec<String> {
        Vec::new()
    }
}

impl CachableBundle<'_, ItarFileIndex> for TestBundle {
    fn get_location(&mut self) -> String {
        "cache-test".into()
    }

    fn get_index_reader(&mut self) -> Result<Box<dyn Read>> {
        self.requested();
        Ok(Box::new(Cursor::new(INDEX)))
    }

    fn index(&mut self) -> &mut ItarFileIndex {
        &mut self.index
    }

    fn initialize_index(&mut self, source: &mut dyn Read) -> Result<()> {
        self.index.initialize(source)
    }

    fn search(&mut self, name: &str) -> Option<ItarFileInfo> {
        self.index.search(name)
    }

    fn open_fileinfo(&mut self, info: &ItarFileInfo, _: &mut dyn StatusBackend) -> OpenResult<InputHandle> {
        self.requested();
        OpenResult::Ok(InputHandle::new_read_only(
            info.name(),
            Cursor::new(b"contents".to_vec()),
            InputOrigin::Other,
        ))
    }
}

fn save_hash(root: &std::path::Path) {
    fs::create_dir_all(root.join("hashes")).unwrap();
    fs::write(root.join("hashes/cache-test"), DigestData::zeros().to_string()).unwrap();
    fs::write(root.join("hashes/cache-test.lock"), "0").unwrap();
}

#[test]
fn cache_only_uses_stale_metadata_without_network_requests() {
    let root = tempfile::tempdir().unwrap();
    save_hash(root.path());
    let data_dir = root.path().join(format!("data/{}", DigestData::zeros()));
    fs::create_dir_all(&data_dir).unwrap();
    fs::write(data_dir.with_extension("index"), INDEX).unwrap();
    fs::write(data_dir.join("file.tex"), b"contents").unwrap();

    let bundle = TestBundle::default();
    let requests = bundle.requests.clone();
    let mut cache = BundleCache::new(Box::new(bundle), true, Some(root.path().into())).unwrap();
    let mut status = NoopStatusBackend {};
    let mut contents = String::new();
    cache
        .input_open_name("file.tex", &mut status)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    assert_eq!(contents, "contents");
    assert!(matches!(
        cache.input_open_name("nested/missing.tex", &mut status),
        OpenResult::NotAvailable
    ));
    assert!(!data_dir.join("nested").exists());
    assert_eq!(requests.get(), 0);
}

#[test]
fn cache_only_does_not_download_missing_hash_or_index() {
    let root = tempfile::tempdir().unwrap();
    let bundle = TestBundle::default();
    let requests = bundle.requests.clone();
    assert!(BundleCache::new(Box::new(bundle), true, Some(root.path().into())).is_err());
    assert_eq!(requests.get(), 0);

    save_hash(root.path());
    let bundle = TestBundle::default();
    let requests = bundle.requests.clone();
    let mut cache = BundleCache::new(Box::new(bundle), true, Some(root.path().into())).unwrap();
    assert!(matches!(cache.input_open_name("file.tex", &mut NoopStatusBackend {}), OpenResult::Err(_)));
    assert_eq!(requests.get(), 0);
}

#[test]
fn loaded_index_is_reused_if_cached_index_is_removed() {
    let root = tempfile::tempdir().unwrap();
    let bundle = TestBundle::default();
    let requests = bundle.requests.clone();
    let mut cache = BundleCache::new(Box::new(bundle), false, Some(root.path().into())).unwrap();
    drop(cache.input_open_name("file.tex", &mut NoopStatusBackend {}).unwrap());
    assert_eq!(requests.get(), 3);

    fs::remove_file(root.path().join(format!("data/{}.index", DigestData::zeros()))).unwrap();
    drop(cache.input_open_name("file.tex", &mut NoopStatusBackend {}).unwrap());
    assert_eq!(requests.get(), 3);
}

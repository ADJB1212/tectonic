use crate::PdfBuildOptions;
use crate::compress::StreamOptions;
use crate::error::{PdfError, PdfErrorKind};
use crate::renderer::{
    Direction, PageRenderer, RenderCommand, RenderFont, RenderSettings, TextRun,
};
use std::{
    cell::Cell,
    collections::{HashMap, VecDeque},
    ffi::{c_char, c_void},
    panic::{AssertUnwindSafe, catch_unwind},
    ptr, slice,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};

/// Timings and queue usage for the most recent PDF engine invocation.
/// Worker timings are summed elapsed times and can overlap coordinator timings.
#[derive(Clone, Debug, Default)]
pub struct PdfBuildStatistics {
    pub font_maps: Duration,
    pub dvi_preparation: Duration,
    pub document_setup: Duration,
    pub interpretation: Duration,
    pub finalization: Duration,
    pub page_rendering: Duration,
    pub predictors: Duration,
    pub compression: Duration,
    pub queue_wait: Duration,
    pub rendered_pages: usize,
    pub serial_page_fallback: bool,
    pub compressed_streams: usize,
    pub peak_queued_bytes: usize,
    pub peak_queued_jobs: usize,
}

#[derive(Default)]
pub(crate) struct WorkerTimings {
    pub rendering: AtomicU64,
    pub predictors: AtomicU64,
    pub compression: AtomicU64,
}

pub(crate) fn record_elapsed(counter: &AtomicU64, start: Instant) {
    counter.fetch_add(
        start.elapsed().as_nanos().min(u64::MAX as u128) as u64,
        Ordering::Relaxed,
    );
}

thread_local! {
    static CURRENT: Cell<*mut Runtime> = const { Cell::new(ptr::null_mut()) };
}

#[derive(Clone, Copy, PartialEq)]
enum CommandKind {
    Bytes,
    Text,
    Rule,
    Graphics,
    Direction,
    ResetFonts,
    Unsupported,
}

struct Command {
    kind: CommandKind,
    font: i32,
    coordinates: [i32; 4],
    start: usize,
    length: usize,
}

struct PagePlan {
    commands: Vec<Command>,
    fonts: Vec<RenderFont>,
    font_ids: HashMap<i32, i32>,
    last_font: Option<(i32, i32)>,
    data: Vec<u8>,
    unit: f64,
    precision: i32,
    autorotate: i32,
    direction: i32,
    current_direction: i32,
}

impl PagePlan {
    fn bytes(&self) -> usize {
        self.data.len()
            + self.commands.len() * std::mem::size_of::<Command>()
            + self.fonts.len() * std::mem::size_of::<RenderFont>()
    }

    fn render(self) -> Result<Vec<u8>, PdfError> {
        let mut output = Vec::new();
        let mut renderer = PageRenderer::new(
            &self.fonts,
            RenderSettings {
                unit: self.unit,
                precision: self.precision,
                autorotate: self.autorotate != 0,
                direction: Direction::try_from(self.direction)?,
            },
            &mut output,
        )?;
        for command in &self.commands {
            let [x, y, width, height] = command.coordinates;
            let data = &self.data[command.start..command.start + command.length];
            renderer.command(match command.kind {
                CommandKind::Bytes => RenderCommand::Bytes(data),
                CommandKind::Text => RenderCommand::Text(TextRun {
                    font: usize::try_from(command.font).map_err(|_| {
                        PdfError::new(
                            PdfErrorKind::MalformedInput,
                            c"Invalid renderer font".to_owned(),
                        )
                    })?,
                    x,
                    y,
                    width,
                    data,
                }),
                CommandKind::Rule => RenderCommand::Rule {
                    x,
                    y,
                    width,
                    height,
                },
                CommandKind::Graphics => RenderCommand::Graphics,
                CommandKind::Direction => RenderCommand::Direction(Direction::try_from(x)?),
                CommandKind::ResetFonts => RenderCommand::ResetFonts { new_page: x != 0 },
                CommandKind::Unsupported => {
                    return Err(PdfError::new(
                        PdfErrorKind::Unsupported,
                        c"Unsupported renderer command".to_owned(),
                    ));
                }
            })?;
        }
        output.push(b'\n');
        Ok(output)
    }
}

struct Pending {
    target: *mut c_void,
    kind: i32,
    bytes: usize,
    receiver: mpsc::Receiver<Result<Vec<u8>, PdfError>>,
}

struct Runtime {
    pool: Option<Arc<rayon::ThreadPool>>,
    max_jobs: usize,
    max_bytes: usize,
    queued_bytes: usize,
    pending: VecDeque<Pending>,
    page: Option<PagePlan>,
    pages_enabled: bool,
    statistics: PdfBuildStatistics,
    timings: Arc<WorkerTimings>,
    phase: Option<(i32, Instant)>,
}

impl Runtime {
    fn has_capacity(&self, bytes: usize) -> bool {
        self.pending.is_empty()
            || (self.pending.len() < self.max_jobs
                && bytes <= self.max_bytes.saturating_sub(self.queued_bytes))
    }

    fn submit(
        &mut self,
        target: *mut c_void,
        kind: i32,
        bytes: usize,
        task: impl FnOnce() -> Result<Vec<u8>, PdfError> + Send + 'static,
    ) {
        let (sender, receiver) = mpsc::channel();
        let pool = self.pool.clone();
        let run = move || {
            let _compression = tectonic_bridge_flate::use_compression_pool(pool);
            let result = catch_unwind(AssertUnwindSafe(|| {
                crate::backend::runtime::catch_backend(task)
            }))
            .unwrap_or_else(|_| {
                Err(PdfError::new(
                    PdfErrorKind::Backend,
                    c"Rust PDF backend failed".to_owned(),
                ))
            });
            let _ = sender.send(result);
        };
        if let Some(pool) = &self.pool {
            pool.spawn(run);
        } else {
            run();
        }
        self.queued_bytes += bytes;
        self.pending.push_back(Pending {
            target,
            kind,
            bytes,
            receiver,
        });
        self.statistics.peak_queued_bytes =
            self.statistics.peak_queued_bytes.max(self.queued_bytes);
        self.statistics.peak_queued_jobs = self.statistics.peak_queued_jobs.max(self.pending.len());
    }
}

fn shared_pool(workers: usize) -> Option<Arc<rayon::ThreadPool>> {
    if workers == 1 {
        return None;
    }
    type CachedPool = Mutex<Option<(usize, Arc<rayon::ThreadPool>)>>;
    static POOL: OnceLock<CachedPool> = OnceLock::new();
    let mut cached = POOL.get_or_init(|| Mutex::new(None)).lock().ok()?;
    if let Some((count, pool)) = &*cached
        && *count == workers
    {
        return Some(Arc::clone(pool));
    }
    let pool = Arc::new(
        rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .thread_name(|index| format!("tectonic-pdf-{index}"))
            .build()
            .ok()?,
    );
    *cached = Some((workers, Arc::clone(&pool)));
    Some(pool)
}

/// Owns the workers and queued data for one native PDF engine invocation.
///
/// Create this guard inside the engine lock and drop it after the native call returns,
/// including after a native abort. Workers never access native document pointers.
pub struct PdfRuntime {
    runtime: Box<Runtime>,
    _compression: tectonic_bridge_flate::CompressionPoolGuard,
}

impl PdfRuntime {
    /// Install a PDF executor on the calling engine thread.
    ///
    /// Panics if an executor is already installed on this thread.
    pub fn new(options: &PdfBuildOptions) -> Self {
        assert!(
            !configured(),
            "PDF engine invocations cannot be nested on one thread"
        );
        let workers = options
            .workers
            .map(usize::from)
            .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, usize::from));
        let pool = shared_pool(workers);
        let compression = tectonic_bridge_flate::use_compression_pool(pool.clone());
        let mut runtime = Box::new(Runtime {
            max_jobs: 2 * pool.as_ref().map_or(1, |pool| pool.current_num_threads()),
            pool,
            max_bytes: options.queued_bytes.get(),
            queued_bytes: 0,
            pending: VecDeque::new(),
            page: None,
            pages_enabled: false,
            statistics: PdfBuildStatistics::default(),
            timings: Arc::new(WorkerTimings::default()),
            phase: None,
        });
        CURRENT.with(|current| current.set(&mut *runtime));
        Self {
            runtime,
            _compression: compression,
        }
    }

    /// Read timings after the native engine has drained its work.
    pub fn statistics(&self) -> PdfBuildStatistics {
        let mut statistics = self.runtime.statistics.clone();
        statistics.page_rendering =
            Duration::from_nanos(self.runtime.timings.rendering.load(Ordering::Relaxed));
        statistics.predictors =
            Duration::from_nanos(self.runtime.timings.predictors.load(Ordering::Relaxed));
        statistics.compression =
            Duration::from_nanos(self.runtime.timings.compression.load(Ordering::Relaxed));
        statistics
    }
}

impl Drop for PdfRuntime {
    fn drop(&mut self) {
        CURRENT.with(|current| current.set(ptr::null_mut()));
        for pending in self.runtime.pending.drain(..) {
            let _ = pending.receiver.recv();
        }
        // SAFETY: Native execution has returned and all worker jobs have finished.
        unsafe {
            crate::clear_input_state();
        }
    }
}

fn with_runtime<T>(default: T, callback: impl FnOnce(&mut Runtime) -> T) -> T {
    CURRENT.with(|current| {
        // SAFETY: Only the coordinator accesses this thread-local pointer. Callbacks
        // never call native code or recursively borrow the runtime.
        unsafe { current.get().as_mut().map_or(default, callback) }
    })
}

pub(crate) fn pool() -> Option<Arc<rayon::ThreadPool>> {
    with_runtime(None, |runtime| runtime.pool.clone())
}

pub(crate) fn configured() -> bool {
    CURRENT.with(|current| !current.get().is_null())
}

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_pipeline_active() -> i32 {
    i32::from(configured())
}

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_phase_start(phase: i32) {
    with_runtime((), |runtime| runtime.phase = Some((phase, Instant::now())));
}

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_phase_end() {
    with_runtime((), |runtime| {
        if let Some((phase, start)) = runtime.phase.take() {
            let duration = start.elapsed();
            match phase {
                0 => runtime.statistics.interpretation += duration,
                1 => runtime.statistics.finalization += duration,
                2 => runtime.statistics.font_maps += duration,
                3 => runtime.statistics.dvi_preparation += duration,
                4 => runtime.statistics.document_setup += duration,
                _ => (),
            }
        }
    });
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_special_supported(data: *const u8, length: usize) -> i32 {
    unsafe { i32::from(special_supported(slice::from_raw_parts(data, length))) }
}

pub(crate) fn special_supported(bytes: &[u8]) -> bool {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    let text = text.trim();
    if !text.split('@').skip(1).all(|suffix| {
        matches!(
            suffix
                .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .next(),
            Some("thispage" | "prevpage" | "nextpage")
        )
    }) {
        return false;
    }
    if text.starts_with("papersize=") || text == "landscape" {
        return true;
    }
    let mut words = text.split_ascii_whitespace();
    if matches!(words.next(), Some("color" | "background")) {
        return true;
    }
    let Some(command) = text
        .strip_prefix("pdf:")
        .and_then(|s| s.split_ascii_whitespace().next())
    else {
        return false;
    };
    matches!(
        command,
        "pagesize"
            | "image"
            | "img"
            | "epdf"
            | "ann"
            | "annot"
            | "annotate"
            | "annotation"
            | "bann"
            | "bannot"
            | "beginann"
            | "eann"
            | "eannot"
            | "endann"
            | "dest"
            | "destination"
            | "outline"
            | "out"
            | "docinfo"
            | "link"
            | "nolink"
            | "begincolor"
            | "bcolor"
            | "bc"
            | "setcolor"
            | "scolor"
            | "sc"
            | "endcolor"
            | "ecolor"
            | "ec"
            | "begingray"
            | "bgray"
            | "bg"
            | "endgray"
            | "egray"
            | "eg"
            | "bgcolor"
            | "bgc"
            | "bbc"
            | "bbg"
    )
}

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_pipeline_enable_pages(enabled: i32) {
    with_runtime((), |runtime| {
        runtime.pages_enabled = enabled != 0;
        runtime.statistics.serial_page_fallback = enabled == 0;
    });
}

pub(crate) fn page_begin(unit: f64, precision: i32, autorotate: i32, direction: i32) -> i32 {
    with_runtime(0, |runtime| {
        if !runtime.pages_enabled {
            return 0;
        }
        runtime.page = Some(PagePlan {
            commands: Vec::new(),
            fonts: Vec::new(),
            font_ids: HashMap::new(),
            last_font: None,
            data: Vec::new(),
            unit,
            precision,
            autorotate,
            direction,
            current_direction: direction,
        });
        1
    })
}

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_page_recording() -> i32 {
    with_runtime(0, |runtime| i32::from(runtime.page.is_some()))
}

pub(crate) fn page_text(font: &RenderFont, id: i32, x: i32, y: i32, width: i32, data: &[u8]) {
    let length = data.len();
    with_runtime((), |runtime| {
        if let Some(page) = &mut runtime.page {
            let font = match page.last_font {
                Some((previous, font)) if previous == id => font,
                _ => {
                    let index = *page.font_ids.entry(id).or_insert_with(|| {
                        page.fonts.push(*font);
                        (page.fonts.len() - 1) as i32
                    });
                    page.last_font = Some((id, index));
                    index
                }
            };
            let start = page.data.len();
            page.data.extend_from_slice(data);
            if page.current_direction == 0
                && let Some(last) = page.commands.last_mut()
                && last.kind == CommandKind::Text
                && last.font == font
                && last.coordinates[1] == y
                && last.coordinates[0].checked_add(last.coordinates[2]) == Some(x)
                && last.start + last.length == start
                && last.length + length <= 1024
                && let Some(combined) = last.coordinates[2].checked_add(width)
            {
                last.coordinates[2] = combined;
                last.length += length;
                return;
            }
            page.commands.push(Command {
                kind: CommandKind::Text,
                font,
                coordinates: [x, y, width, 0],
                start,
                length,
            });
        }
    });
}

pub(crate) fn page_command(kind: i32, x: i32, y: i32, width: i32, height: i32) {
    let kind = match kind {
        2 => CommandKind::Rule,
        3 => CommandKind::Graphics,
        4 => CommandKind::Direction,
        5 => CommandKind::ResetFonts,
        _ => CommandKind::Unsupported,
    };
    with_runtime((), |runtime| {
        if let Some(page) = &mut runtime.page {
            if kind == CommandKind::Direction {
                page.current_direction = x;
            }
            page.commands.push(Command {
                kind,
                font: 0,
                coordinates: [x, y, width, height],
                start: 0,
                length: 0,
            });
        }
    });
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_page_bytes(data: *const c_char, length: usize) -> i32 {
    unsafe {
        with_runtime(0, |runtime| {
            let Some(page) = &mut runtime.page else {
                return 0;
            };
            let start = page.data.len();
            page.data
                .extend_from_slice(slice::from_raw_parts(data.cast(), length));
            if let Some(last) = page.commands.last_mut()
                && last.kind == CommandKind::Bytes
                && last.start + last.length == start
            {
                last.length += length;
                return 1;
            }
            page.commands.push(Command {
                kind: CommandKind::Bytes,
                font: 0,
                coordinates: [0; 4],
                start,
                length,
            });
            1
        })
    }
}

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_page_size() -> usize {
    with_runtime(0, |runtime| {
        runtime.page.as_ref().map_or(0, PagePlan::bytes)
    })
}

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_pipeline_has_capacity(bytes: usize) -> i32 {
    with_runtime(1, |runtime| i32::from(runtime.has_capacity(bytes)))
}

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_page_submit(target: *mut c_void, compression_level: u32) -> i32 {
    with_runtime(0, |runtime| {
        let Some(page) = runtime.page.take() else {
            return 0;
        };
        let pool = runtime.pool.clone();
        let timings = Arc::clone(&runtime.timings);
        runtime.statistics.rendered_pages += 1;
        runtime.statistics.compressed_streams += usize::from(compression_level > 0);
        runtime.submit(
            target,
            i32::from(compression_level > 0),
            page.bytes(),
            move || {
                let start = Instant::now();
                let rendered = page.render()?;
                record_elapsed(&timings.rendering, start);
                if compression_level > 0 {
                    crate::compress::compress_owned(
                        rendered,
                        StreamOptions {
                            level: compression_level,
                            predictor: 0,
                            columns: 0,
                            bpc: 0,
                            colors: 0,
                        },
                        pool.as_deref(),
                        &timings,
                    )
                    .map_err(|_| {
                        PdfError::new(
                            PdfErrorKind::Backend,
                            c"PDF stream compression failed".to_owned(),
                        )
                    })
                } else {
                    Ok(rendered)
                }
            },
        );
        1
    })
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_stream_submit(
    target: *mut c_void,
    data: *const u8,
    length: usize,
    level: u32,
    predictor: i32,
    columns: i32,
    bpc: i32,
    colors: i32,
) -> i32 {
    unsafe {
        with_runtime(0, |runtime| {
            let input = slice::from_raw_parts(data, length).to_vec();
            let pool = runtime.pool.clone();
            let timings = Arc::clone(&runtime.timings);
            runtime.statistics.compressed_streams += 1;
            runtime.submit(target, 1, length, move || {
                crate::compress::compress_owned(
                    input,
                    StreamOptions {
                        level,
                        predictor,
                        columns,
                        bpc,
                        colors,
                    },
                    pool.as_deref(),
                    &timings,
                )
                .map_err(|_| {
                    PdfError::new(
                        PdfErrorKind::Backend,
                        c"PDF stream compression failed".to_owned(),
                    )
                })
            });
            1
        })
    }
}

struct TaskResult {
    data: Result<Vec<u8>, PdfError>,
}

#[repr(C)]
struct NativeResult {
    target: *mut c_void,
    data: *const u8,
    length: usize,
    kind: i32,
    status: i32,
    handle: *mut c_void,
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_pipeline_take(result: *mut NativeResult) -> i32 {
    unsafe {
        with_runtime(0, |runtime| {
            let Some(pending) = runtime.pending.pop_front() else {
                return 0;
            };
            let start = Instant::now();
            let completed = Box::new(TaskResult {
                data: pending.receiver.recv().unwrap_or_else(|_| {
                    Err(PdfError::new(
                        PdfErrorKind::InvalidState,
                        c"PDF worker stopped".to_owned(),
                    ))
                }),
            });
            runtime.statistics.queue_wait += start.elapsed();
            runtime.queued_bytes -= pending.bytes;
            let (data, length, status) = match &completed.data {
                Ok(data) => (data.as_ptr(), data.len(), 0),
                Err(error) => {
                    crate::backend::runtime::store_error(error);
                    (ptr::null(), 0, -1)
                }
            };
            *result = NativeResult {
                target: pending.target,
                data,
                length,
                kind: pending.kind,
                status,
                handle: Box::into_raw(completed).cast(),
            };
            1
        })
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_pipeline_release(result: *mut c_void) {
    unsafe {
        drop(Box::from_raw(result.cast::<TaskResult>()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;

    fn options(workers: usize, bytes: usize) -> PdfBuildOptions {
        PdfBuildOptions {
            workers: NonZeroUsize::new(workers),
            queued_bytes: NonZeroUsize::new(bytes).unwrap(),
        }
    }

    #[test]
    #[should_panic(expected = "PDF engine invocations cannot be nested")]
    fn nested_runtime_installation_is_rejected_before_replacing_live_state() {
        let _first = PdfRuntime::new(&options(1, 100));
        let _second = PdfRuntime::new(&options(1, 100));
    }

    #[test]
    fn independent_jobs_overlap_but_results_are_taken_in_submission_order() {
        let guard = PdfRuntime::new(&options(2, 1024));
        if guard.runtime.pool.is_none() {
            return;
        }
        let (started, observed) = mpsc::channel();
        let (release, blocked) = mpsc::channel();
        with_runtime((), |runtime| {
            runtime.submit(ptr::null_mut(), 0, 10, move || {
                blocked.recv().unwrap();
                Ok(vec![1])
            });
            runtime.submit(ptr::null_mut(), 0, 10, move || {
                started.send(()).unwrap();
                Ok(vec![2])
            });
        });
        observed
            .recv_timeout(std::time::Duration::from_secs(10))
            .unwrap();
        release.send(()).unwrap();
        with_runtime((), |runtime| {
            assert_eq!(
                runtime
                    .pending
                    .pop_front()
                    .unwrap()
                    .receiver
                    .recv()
                    .unwrap()
                    .unwrap(),
                vec![1]
            );
            assert_eq!(
                runtime
                    .pending
                    .pop_front()
                    .unwrap()
                    .receiver
                    .recv()
                    .unwrap()
                    .unwrap(),
                vec![2]
            );
        });
    }

    #[test]
    fn admission_bounds_jobs_and_bytes_and_allows_one_oversized_job() {
        let guard = PdfRuntime::new(&options(1, 100));
        with_runtime((), |runtime| {
            assert!(runtime.has_capacity(1000));
            runtime.submit(ptr::null_mut(), 0, 60, || Ok(vec![]));
            assert!(!runtime.has_capacity(41));
            assert!(runtime.has_capacity(40));
            runtime.submit(ptr::null_mut(), 0, 40, || Ok(vec![]));
            assert!(!runtime.has_capacity(0));
        });
        drop(guard);
        assert!(!configured());
    }

    #[test]
    fn typed_worker_errors_retain_context_until_native_result_release() {
        for workers in [1, 2] {
            let _runtime = PdfRuntime::new(&options(workers, 100));
            with_runtime((), |runtime| {
                runtime.submit(ptr::null_mut(), 0, 10, || {
                    Err(
                        PdfError::new(PdfErrorKind::MalformedInput, c"bad page".to_owned())
                            .at(23)
                            .in_file(b"page\xff.xdv"),
                    )
                });
            });
            let mut result = std::mem::MaybeUninit::<NativeResult>::uninit();
            // SAFETY: take initializes the result, whose owner is released after inspection.
            unsafe {
                assert_eq!(tt_pdf_pipeline_take(result.as_mut_ptr()), 1);
                let result = result.assume_init();
                assert_eq!(result.status, -1);
                assert!(result.data.is_null());
                let error = (&*result.handle.cast::<TaskResult>())
                    .data
                    .as_ref()
                    .unwrap_err();
                assert_eq!(error.kind, PdfErrorKind::MalformedInput);
                assert_eq!(error.offset, Some(23));
                assert_eq!(error.file.as_deref(), Some(&b"page\xff.xdv"[..]));
                assert_eq!(
                    std::ffi::CStr::from_ptr(crate::backend::runtime::tt_pdf_last_error()),
                    c"bad page"
                );
                tt_pdf_pipeline_release(result.handle);
            }
            with_runtime((), |runtime| {
                assert_eq!(runtime.queued_bytes, 0);
                runtime.submit(ptr::null_mut(), 0, 1, || Ok(vec![1]));
                assert_eq!(
                    runtime
                        .pending
                        .pop_front()
                        .unwrap()
                        .receiver
                        .recv()
                        .unwrap()
                        .unwrap(),
                    vec![1]
                );
            });
        }
    }

    #[test]
    fn failed_jobs_and_native_abort_cleanup_leave_the_executor_reusable() {
        let guard = PdfRuntime::new(&options(2, 100));
        let (completed, observed) = mpsc::channel();
        with_runtime((), |runtime| {
            runtime.submit(ptr::null_mut(), 0, 1, || panic!("injected worker failure"));
            runtime.submit(ptr::null_mut(), 0, 1, move || {
                completed.send(()).unwrap();
                Ok(vec![])
            });
            let error = runtime
                .pending
                .pop_front()
                .unwrap()
                .receiver
                .recv()
                .unwrap()
                .unwrap_err();
            assert_eq!(error.kind, PdfErrorKind::Backend);
        });
        drop(guard);
        observed.recv().unwrap();
        assert!(!configured());
        let _next = PdfRuntime::new(&options(1, 100));
        assert!(configured());
    }

    #[test]
    fn special_classifier_keeps_unknown_and_mutable_named_objects_on_the_legacy_path() {
        for (text, supported) in [
            ("color push rgb 1 0 0", true),
            ("pdf:dest (next) [ @nextpage /Fit ]", true),
            ("pdf:image (picture.png)", true),
            ("pdf:obj @mutable <<>>", false),
            ("pdf:dest (next) [ @nextpage_custom /Fit ]", false),
            ("pdf:literal direct 1 0 0 rg", false),
            ("ps: arbitrary PostScript", false),
        ] {
            // SAFETY: The classifier borrows a valid string for this call.
            assert_eq!(
                unsafe { tt_pdf_special_supported(text.as_ptr(), text.len()) } != 0,
                supported,
                "{text}"
            );
        }
    }
}

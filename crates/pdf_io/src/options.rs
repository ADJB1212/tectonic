use std::num::NonZeroUsize;

/// Resource limits for building one PDF.
#[derive(Clone, Debug)]
pub struct PdfBuildOptions {
    /// Worker count. None uses the available CPU count.
    pub workers: Option<NonZeroUsize>,
    /// Maximum queued payload size, excluding active jobs and document storage.
    pub queued_bytes: NonZeroUsize,
}

impl Default for PdfBuildOptions {
    fn default() -> Self {
        Self {
            workers: None,
            queued_bytes: NonZeroUsize::new(64 * 1024 * 1024).unwrap(),
        }
    }
}

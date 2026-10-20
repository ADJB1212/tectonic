// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! Bounded concurrent byte-range reads for network bundles.

use std::{
    sync::{
        Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
};
use tectonic_geturl::{DefaultBackend, DefaultRangeReader, GetUrlBackend};
use tectonic_io_base::OpenResult;
use tectonic_status_base::{NoopStatusBackend, StatusBackend, tt_note};

const DEFAULT_PREFETCH_CONCURRENCY: usize = 16;
const MAX_PREFETCH_CONCURRENCY: usize = 64;
const MAX_PREFETCH_BYTES: usize = 64 * 1024 * 1024;

fn prefetch_concurrency(value: Option<&str>, file_count: usize) -> usize {
    if file_count == 0 {
        return 0;
    }

    value
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_PREFETCH_CONCURRENCY)
        .min(MAX_PREFETCH_CONCURRENCY)
        .min(file_count)
}

fn prefetch_indices(lengths: impl Iterator<Item = usize>) -> Vec<usize> {
    let mut bytes = 0usize;
    let mut selected = Vec::new();

    for (index, length) in lengths.enumerate() {
        if let Some(next_bytes) = bytes.checked_add(length)
            && next_bytes <= MAX_PREFETCH_BYTES
        {
            bytes = next_bytes;
            selected.push(index);
        }
    }

    selected
}

pub(crate) fn open<I: Sync>(
    url: &str, infos: &[I], size: fn(&I) -> usize, read_file: fn(&mut DefaultRangeReader, &I, &mut dyn StatusBackend) -> OpenResult<Vec<u8>>,
    status: &mut dyn StatusBackend,
) -> Vec<OpenResult<Vec<u8>>> {
    if infos.is_empty() {
        return Vec::new();
    }

    let selected = prefetch_indices(infos.iter().map(size));
    if selected.is_empty() {
        return infos.iter().map(|_| OpenResult::NotAvailable).collect();
    }

    let configured = std::env::var("TECTONIC_PREFETCH_CONCURRENCY").ok();
    let concurrency = prefetch_concurrency(configured.as_deref(), selected.len());
    tt_note!(
        status,
        "prefetching {} of {} files ({}-way concurrent)",
        selected.len(),
        infos.len(),
        concurrency
    );

    let next = AtomicUsize::new(0);
    let abort = AtomicBool::new(false);
    let result_slots: Vec<Mutex<OpenResult<Vec<u8>>>> = (0..infos.len()).map(|_| Mutex::new(OpenResult::NotAvailable)).collect();

    thread::scope(|scope| {
        let mut workers = Vec::with_capacity(concurrency);
        for _ in 0..concurrency {
            let worker = thread::Builder::new().spawn_scoped(scope, || {
                let mut reader = DefaultBackend::default().open_range_reader(url);
                let mut status = NoopStatusBackend {};

                loop {
                    if abort.load(Ordering::Relaxed) {
                        break;
                    }

                    let selected_index = next.fetch_add(1, Ordering::Relaxed);
                    if selected_index >= selected.len() {
                        break;
                    }
                    let i = selected[selected_index];

                    let result = read_file(&mut reader, &infos[i], &mut status);
                    if !matches!(result, OpenResult::Ok(_)) {
                        abort.store(true, Ordering::Relaxed);
                    }
                    if let Ok(mut slot) = result_slots[i].lock() {
                        *slot = result;
                    }
                }
            });

            match worker {
                Ok(worker) => workers.push(worker),
                Err(_) => break,
            }
        }

        // A failed worker leaves its unclaimed slots uncached. The normal
        // on-demand path will retry them without failing the build.
        for worker in workers {
            let _ = worker.join();
        }
    });

    result_slots
        .into_iter()
        .map(|slot| slot.into_inner().unwrap_or(OpenResult::NotAvailable))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::mpsc, time::Duration};

    #[test]
    fn downloads_overlap_with_bounded_workers_and_ordered_results() {
        struct Info {
            index: u8,
            started: mpsc::Sender<u8>,
            release: Mutex<mpsc::Receiver<()>>,
        }

        let (started, received) = mpsc::channel();
        let mut releases = Vec::new();
        let infos: Vec<_> = (0..4)
            .map(|index| {
                let (release, wait) = mpsc::channel();
                releases.push(release);
                Info {
                    index,
                    started: started.clone(),
                    release: Mutex::new(wait),
                }
            })
            .collect();
        let configured = std::env::var("TECTONIC_PREFETCH_CONCURRENCY").ok();
        let concurrency = prefetch_concurrency(configured.as_deref(), 3);

        thread::scope(|scope| {
            let controller = scope.spawn(move || {
                for _ in 0..concurrency {
                    received.recv_timeout(Duration::from_secs(5)).unwrap();
                }
                assert!(matches!(received.try_recv(), Err(mpsc::TryRecvError::Empty)));
                for release in releases.into_iter().rev() {
                    release.send(()).unwrap();
                }
            });

            let results = open(
                "https://example.invalid/bundle",
                &infos,
                |info| if info.index == 2 { usize::MAX } else { 1 },
                |_, info, _| {
                    info.started.send(info.index).unwrap();
                    info.release.lock().unwrap().recv_timeout(Duration::from_secs(5)).unwrap();
                    OpenResult::Ok(vec![info.index])
                },
                &mut NoopStatusBackend {},
            );

            controller.join().unwrap();
            assert_eq!(results.len(), infos.len());
            for (index, result) in results.into_iter().enumerate() {
                if index == 2 {
                    assert!(matches!(result, OpenResult::NotAvailable));
                } else {
                    assert_eq!(result.unwrap(), vec![index as u8]);
                }
            }
        });
    }

    #[test]
    fn prefetch_concurrency_is_bounded() {
        assert_eq!(prefetch_concurrency(None, 100), 16);
        assert_eq!(prefetch_concurrency(Some("4"), 100), 4);
        assert_eq!(prefetch_concurrency(Some("0"), 100), 16);
        assert_eq!(prefetch_concurrency(Some("invalid"), 100), 16);
        assert_eq!(prefetch_concurrency(Some("1000"), 1000), 64);
        assert_eq!(prefetch_concurrency(Some("1000"), 3), 3);
        assert_eq!(prefetch_concurrency(Some("4"), 0), 0);
    }

    #[test]
    fn prefetch_selection_caps_total_bytes() {
        let mib = 1024 * 1024;
        let lengths = [40 * mib, 30 * mib, 20 * mib, usize::MAX];
        assert_eq!(prefetch_indices(lengths.into_iter()), vec![0, 2]);
    }
}

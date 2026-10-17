// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

use super::Shaper;
use crate::c_api::XeTeXLayoutEngine;
use rayon::prelude::*;
use std::{collections::HashMap, ffi::CStr, ptr, slice, sync::OnceLock, thread};
use tectonic_bridge_harfbuzz as hb;

pub(super) struct PreparedBuffer(pub(super) hb::Buffer);

// SAFETY: Each buffer is created, shaped, and transferred with exclusive ownership.
unsafe impl Send for PreparedBuffer {}

struct ShapingFont<'a> {
    font: hb::FontRef<'a>,
    script: hb::Tag,
    language: hb::Language,
    features: &'a [hb::Feature],
    vertical: bool,
}

// SAFETY: The font is immutable, its FreeType callbacks lock their face, and
// workers use separate buffers. The owning engine stays paused until they finish.
unsafe impl Sync for ShapingFont<'_> {}

impl ShapingFont<'_> {
    fn shape(&self, text: &[u16]) -> Option<PreparedBuffer> {
        let mut buffer = hb::Buffer::new();
        buffer.as_mut().add_utf16(text);
        buffer.as_mut().set_direction(if self.vertical {
            hb::Direction::Ttb
        } else {
            hb::Direction::Ltr
        });
        buffer.as_mut().set_script(self.script.to_script());
        buffer.as_mut().set_language(self.language);
        buffer.as_mut().guess_segment_properties();
        let properties = buffer.as_mut().get_segment_properties();
        let shapers = [c"ot".as_ptr(), ptr::null()];
        let mut plan =
            hb::ShapePlan::new_cached(self.font.face(), &properties, self.features, Some(&shapers));
        if plan
            .as_mut()
            .execute(self.font, buffer.as_mut(), self.features)
        {
            Some(PreparedBuffer(buffer))
        } else {
            None
        }
    }
}

fn pool() -> Option<&'static rayon::ThreadPool> {
    static POOL: OnceLock<Option<rayon::ThreadPool>> = OnceLock::new();
    POOL.get_or_init(|| {
        let available = thread::available_parallelism().map_or(1, usize::from);
        let threads = std::env::var("TECTONIC_LAYOUT_THREADS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|&value| value > 0)
            .unwrap_or(available)
            .min(available);
        if threads < 2 {
            return None;
        }
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads.min(8))
            .thread_name(|index| format!("tectonic-layout-{index}"))
            .build()
            .ok()
    })
    .as_ref()
}

#[repr(C)]
pub struct NativeLayoutRequest {
    engine: XeTeXLayoutEngine,
    text: *const u16,
    length: usize,
}

pub struct PreparedLayouts {
    engines: Vec<XeTeXLayoutEngine>,
}

/// Prepare independent ASCII runs for the paragraph's final line packing.
///
/// # Safety
/// Requests must contain live engines and text buffers. Engines must remain
/// exclusively accessible to this call and alive until `ttxl_release_prepared_layouts`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ttxl_prepare_layouts(
    requests: *const NativeLayoutRequest,
    count: usize,
) -> *mut PreparedLayouts {
    unsafe {
        if count < 4 {
            return ptr::null_mut();
        }
        let requests = slice::from_raw_parts(requests, count);
        if requests.iter().map(|request| request.length).sum::<usize>() < 256 {
            return ptr::null_mut();
        }
        let Some(pool) = pool() else {
            return ptr::null_mut();
        };
        prepare_layouts(requests, pool)
    }
}

unsafe fn prepare_layouts(
    requests: &[NativeLayoutRequest],
    pool: &rayon::ThreadPool,
) -> *mut PreparedLayouts {
    unsafe {
        let mut engines = Vec::new();
        let mut indices = HashMap::new();
        let mut work = Vec::new();
        for request in requests {
            let engine = &*request.engine;
            if !matches!(engine.shaper, Shaper::OpenType)
                || engine.shaper_list.len() != 2
                || CStr::from_ptr(engine.shaper_list[0]) != c"ot"
            {
                continue;
            }
            let text = slice::from_raw_parts(request.text, request.length);
            if !text.iter().all(|&ch| (32..127).contains(&ch))
                || !text.iter().any(|&ch| (ch as u8).is_ascii_alphabetic())
            {
                continue;
            }
            let index = *indices.entry(request.engine as usize).or_insert_with(|| {
                engines.push(request.engine);
                engines.len() - 1
            });
            work.push((index, text.to_vec()));
        }
        if work.len() < 4 {
            return ptr::null_mut();
        }
        let results = {
            let fonts: Vec<_> = engines
                .iter()
                .map(|&engine| {
                    let engine = &*engine;
                    let font = engine.font().hb_font();
                    hb::sys::hb_font_make_immutable(font.as_ptr());
                    ShapingFont {
                        font,
                        script: engine.script,
                        language: engine.language,
                        features: &engine.features,
                        vertical: engine.font().layout_dir_vertical(),
                    }
                })
                .collect();
            pool.install(|| {
                work.par_iter()
                    .map(|(index, text)| fonts[*index].shape(text))
                    .collect::<Vec<_>>()
            })
        };
        for ((index, text), result) in work.into_iter().zip(results) {
            if let Some(buffer) = result {
                (*engines[index]).prepared_layouts.insert(text, buffer);
            }
        }
        Box::into_raw(Box::new(PreparedLayouts { engines }))
    }
}

/// Release unused prepared buffers after packing a paragraph.
///
/// # Safety
/// `prepared` must be null or a live handle returned by `ttxl_prepare_layouts`.
/// All referenced engines must still be alive and exclusively accessible.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ttxl_release_prepared_layouts(prepared: *mut PreparedLayouts) {
    unsafe {
        if !prepared.is_null() {
            let prepared = Box::from_raw(prepared);
            for &engine in &prepared.engines {
                (*engine).prepared_layouts.clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        path::Path,
        sync::{Arc, Mutex},
    };
    use tectonic_bridge_freetype2 as ft;

    fn glyphs(buffer: &PreparedBuffer) -> Vec<(u32, u32, i32, i32, i32, i32)> {
        let buffer = buffer.0.as_ref();
        buffer
            .glyph_info()
            .unwrap()
            .iter()
            .zip(buffer.glyph_positions().unwrap())
            .map(|(glyph, position)| {
                (
                    glyph.codepoint,
                    glyph.cluster,
                    position.x_advance,
                    position.y_advance,
                    position.x_offset,
                    position.y_offset,
                )
            })
            .collect()
    }

    #[test]
    fn worker_count_preserves_glyphs_clusters_advances_and_offsets() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/assets/lmroman12-regular.otf");
        let face = Arc::new(Mutex::new(
            ft::Face::new_memory(std::fs::read(path).unwrap(), 0).unwrap(),
        ));
        let tables = Arc::clone(&face);
        let mut hb_face = hb::Face::new_tables(move |_, tag| {
            tables
                .lock()
                .unwrap()
                .load_sfnt_table(ft::TableTag::Other(tag.to_raw()))
                .ok()
                .map(hb::Blob::new)
        });
        let upem = face.lock().unwrap().units_per_em();
        hb_face.as_mut().set_upem(u32::from(upem));
        let mut font = hb::Font::new(hb_face.as_ref());
        font.as_mut().set_funcs(crate::font::get_font_funcs(), face);
        font.as_mut().set_scale(i32::from(upem), i32::from(upem));
        // SAFETY: The exclusively owned font is fully configured before workers borrow it.
        unsafe {
            hb::sys::hb_font_make_immutable(font.as_ref().as_ptr());
        }
        let shaper = ShapingFont {
            font: font.as_ref(),
            script: hb::Tag::new(0),
            language: hb::Language::default(),
            features: &[],
            vertical: false,
        };
        let texts: Vec<Vec<u16>> = (0..16)
            .map(|index| {
                format!("office affinity -- final line {index} with kerning: AVATAR To Wa ffi.")
                    .encode_utf16()
                    .collect()
            })
            .collect();
        let expected: Vec<_> = texts
            .iter()
            .map(|text| glyphs(&shaper.shape(text).unwrap()))
            .collect();
        for workers in [1, 3] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap();
            let actual = pool.install(|| {
                texts
                    .par_iter()
                    .map(|text| shaper.shape(text).unwrap())
                    .collect::<Vec<_>>()
            });
            for (actual, expected) in actual.iter().zip(&expected) {
                assert_eq!(glyphs(actual), *expected);
            }
        }
        // SAFETY: An empty batch does not dereference the request pointer.
        assert!(unsafe { ttxl_prepare_layouts(ptr::null(), 0) }.is_null());
    }
}

use crate::engine::LayoutEngine;
use crate::font::Font;
use std::{cell::RefCell, collections::HashMap};
#[cfg(not(target_os = "macos"))]
use tectonic_bridge_fontconfig as fc;

mod engine;
mod font;
mod manager;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct FloatPoint {
    pub x: f32,
    pub y: f32,
}

/// cbindgen:rename-all=camelCase
#[derive(Copy, Clone, Default, PartialEq, Debug)]
#[repr(C)]
pub struct GlyphBBox {
    pub x_min: f32,
    pub y_min: f32,
    pub x_max: f32,
    pub y_max: f32,
}

#[cfg(not(target_os = "macos"))]
pub type Fixed = i32;
/// cbindgen:ignore
#[cfg(target_os = "macos")]
pub type Fixed = u32;
pub type OTTag = u32;
pub type GlyphID = u16;
/// cbindgen:ignore
pub type XeTeXFont = *mut Font;
/// cbindgen:ignore
pub type XeTeXLayoutEngine = *mut LayoutEngine;
#[cfg(not(target_os = "macos"))]
pub(crate) type RawPlatformFontRef = *mut fc::sys::FcPattern;
#[cfg(target_os = "macos")]
pub(crate) type RawPlatformFontRef = tectonic_mac_core::sys::CTFontDescriptorRef;
/// cbindgen:ignore
#[cfg(not(target_os = "macos"))]
pub(crate) type PlatformFontRef = fc::Pattern;
/// cbindgen:ignore
#[cfg(target_os = "macos")]
pub(crate) type PlatformFontRef = tectonic_mac_core::CTFontDescriptor;

thread_local! {
    static GLYPH_BOXES: RefCell<HashMap<(u16, u16), GlyphBBox>> = RefCell::new(HashMap::new());
    static LEFT_PROT: RefCell<HashMap<(i32, u32), i32>> = RefCell::new(HashMap::new());
    static RIGHT_PROT: RefCell<HashMap<(i32, u32), i32>> = RefCell::new(HashMap::new());
}

pub(crate) fn clear_font_caches() {
    GLYPH_BOXES.set(HashMap::new());
    LEFT_PROT.set(HashMap::new());
    RIGHT_PROT.set(HashMap::new());
}

#[no_mangle]
pub unsafe extern "C" fn getCachedGlyphBBox(
    font_id: u16,
    glyph_id: u16,
    bbox: *mut GlyphBBox,
) -> i32 {
    GLYPH_BOXES.with_borrow(|cache| match cache.get(&(font_id, glyph_id)) {
        Some(val) => {
            *bbox = *val;
            1
        }
        None => 0,
    })
}

#[no_mangle]
pub unsafe extern "C" fn cacheGlyphBBox(font_id: u16, glyph_id: u16, bbox: *const GlyphBBox) {
    GLYPH_BOXES.with_borrow_mut(|cache| cache.insert((font_id, glyph_id), *bbox));
}

/* The following code used to be in a file called "hz.cpp" and there's no
 * particular reason for it to be here, but it was a tiny file with a weird
 * name so I wanted to get rid of it. The functions are invoked from the C
 * code. */

pub const LEFT_SIDE: i32 = 0;
pub const RIGHT_SIDE: i32 = 1;

#[no_mangle]
pub extern "C" fn set_cp_code(font_num: i32, code: u32, side: i32, value: i32) {
    match side {
        LEFT_SIDE => LEFT_PROT.with_borrow_mut(|cache| cache.insert((font_num, code), value)),
        RIGHT_SIDE => RIGHT_PROT.with_borrow_mut(|cache| cache.insert((font_num, code), value)),
        _ => unreachable!(),
    };
}

#[no_mangle]
pub extern "C" fn get_cp_code(font_num: i32, code: u32, side: i32) -> i32 {
    match side {
        LEFT_SIDE => LEFT_PROT.with_borrow(|cache| cache.get(&(font_num, code)).copied()),
        RIGHT_SIDE => RIGHT_PROT.with_borrow(|cache| cache.get(&(font_num, code)).copied()),
        _ => unreachable!(),
    }
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_teardown_discards_cached_metrics_and_protrusion_codes() {
        let bounds = GlyphBBox {
            x_min: -1.0,
            y_min: -2.0,
            x_max: 3.0,
            y_max: 4.0,
        };
        let mut actual = GlyphBBox::default();
        unsafe {
            cacheGlyphBBox(7, 42, &bounds);
            assert_eq!(getCachedGlyphBBox(7, 42, &mut actual), 1);
        }
        assert_eq!(actual, bounds);
        set_cp_code(7, 42, LEFT_SIDE, 100);
        set_cp_code(7, 42, RIGHT_SIDE, 200);
        assert_eq!(get_cp_code(7, 42, LEFT_SIDE), 100);
        assert_eq!(get_cp_code(7, 42, RIGHT_SIDE), 200);

        crate::manager::FontManager::destroy();
        assert_eq!(unsafe { getCachedGlyphBBox(7, 42, &mut actual) }, 0);
        assert_eq!(get_cp_code(7, 42, LEFT_SIDE), 0);
        assert_eq!(get_cp_code(7, 42, RIGHT_SIDE), 0);
    }
}

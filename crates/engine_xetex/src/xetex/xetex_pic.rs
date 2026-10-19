#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::xetex_xetexd::{
    cur_list, help_ptr, mem, memory_word, name_of_file, rust_input_handle_t,
};

use libc::{c_char, c_int, c_uint, c_void, size_t};
use std::f64::consts::PI;
use std::ptr;

use super::xetex_checkpoint::*;
use super::xetex_constants::*;
use super::xetex_ext::*;
use super::xetex_xetex_ot_math::{D2Fix, Fix2D};
use super::xetex_xetexd::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct real_point {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct transform_t {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct real_rect {
    pub x: f32,
    pub y: f32,
    pub wd: f32,
    pub ht: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct pdf_tmatrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct pdf_rect {
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct pdf_coord {
    pub x: f64,
    pub y: f64,
}

unsafe extern "C" {
    fn pdf_open(filename: *const c_char, handle: rust_input_handle_t) -> *mut c_void;
    fn pdf_close(pf: *mut c_void);
    fn pdf_doc_get_page_count(pf: *mut c_void) -> c_int;
    fn pdf_doc_get_page(
        pf: *mut c_void,
        page_no: c_int,
        options: c_int,
        bbox: *mut pdf_rect,
        matrix: *mut pdf_tmatrix,
        page_obj: *mut *mut c_void,
    ) -> *mut c_void;
    fn pdf_release_obj(obj: *mut c_void);
    fn pdf_dev_transform(p: *mut pdf_coord, m: *const pdf_tmatrix);

    fn check_for_jpeg(handle: rust_input_handle_t) -> c_int;
    fn jpeg_get_bbox(
        handle: rust_input_handle_t,
        width: *mut c_uint,
        height: *mut c_uint,
        xdensity: *mut f64,
        ydensity: *mut f64,
    ) -> c_int;

    fn check_for_bmp(handle: rust_input_handle_t) -> c_int;
    fn bmp_get_bbox(
        handle: rust_input_handle_t,
        width: *mut c_uint,
        height: *mut c_uint,
        xdensity: *mut f64,
        ydensity: *mut f64,
    ) -> c_int;

    fn check_for_png(handle: rust_input_handle_t) -> c_int;
    fn png_get_bbox(
        handle: rust_input_handle_t,
        width: *mut c_uint,
        height: *mut c_uint,
        xdensity: *mut f64,
        ydensity: *mut f64,
    ) -> c_int;
}

#[inline]
fn min4(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a.min(b).min(c).min(d)
}

#[inline]
fn max4(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a.max(b).max(c).max(d)
}

#[unsafe(export_name = "count_pdf_file_pages")]
pub unsafe extern "C" fn count_pdf_file_pages() -> c_int {
    unsafe {
        tt_checkpoint_disallow();

        let handle =
            super::bridge::ttstub_input_open(name_of_file, super::bridge::TTBC_FILE_FORMAT_PICT, 0);
        if handle == super::bridge::INVALID_HANDLE {
            return 0;
        }

        let pf = pdf_open(name_of_file, handle);
        if pf.is_null() {
            super::bridge::ttstub_input_close(handle);
            return 0;
        }

        let pages = pdf_doc_get_page_count(pf);
        pdf_close(pf);
        super::bridge::ttstub_input_close(handle);
        pages
    }
}

unsafe fn pdf_get_rect(
    filename: *mut c_char,
    handle: rust_input_handle_t,
    mut page_num: c_int,
    pdf_box: c_int,
    box_rect: *mut real_rect,
) -> c_int {
    unsafe {
        let pf = pdf_open(filename, handle);
        if pf.is_null() {
            return -1;
        }

        let pages = pdf_doc_get_page_count(pf);

        if page_num > pages {
            page_num = pages;
        }
        if page_num < 0 {
            page_num = pages + 1 + page_num;
        }
        if page_num < 1 {
            page_num = 1;
        }

        let dpx_options: c_int = match pdf_box {
            pdfbox_media => 2,
            pdfbox_bleed => 5,
            pdfbox_trim => 4,
            pdfbox_art => 3,
            _ => 1, // pdfbox_crop and default
        };

        let mut bbox = pdf_rect::default();
        let mut matrix = pdf_tmatrix::default();
        let page = pdf_doc_get_page(
            pf,
            page_num,
            dpx_options,
            &mut bbox,
            &mut matrix,
            ptr::null_mut(),
        );
        pdf_close(pf);

        if page.is_null() {
            return -1;
        }

        pdf_release_obj(page);

        let mut p1 = pdf_coord {
            x: bbox.llx,
            y: bbox.lly,
        };
        pdf_dev_transform(&mut p1, &matrix);

        let mut p2 = pdf_coord {
            x: bbox.urx,
            y: bbox.lly,
        };
        pdf_dev_transform(&mut p2, &matrix);

        let mut p3 = pdf_coord {
            x: bbox.urx,
            y: bbox.ury,
        };
        pdf_dev_transform(&mut p3, &matrix);

        let mut p4 = pdf_coord {
            x: bbox.llx,
            y: bbox.ury,
        };
        pdf_dev_transform(&mut p4, &matrix);

        bbox.llx = min4(p1.x, p2.x, p3.x, p4.x);
        bbox.lly = min4(p1.y, p2.y, p3.y, p4.y);
        bbox.urx = max4(p1.x, p2.x, p3.x, p4.x);
        bbox.ury = max4(p1.y, p2.y, p3.y, p4.y);

        let scale = 72.27 / 72.0;
        (*box_rect).x = (scale * bbox.llx) as f32;
        (*box_rect).y = (scale * bbox.lly) as f32;
        (*box_rect).wd = (scale * (bbox.urx - bbox.llx)) as f32;
        (*box_rect).ht = (scale * (bbox.ury - bbox.lly)) as f32;

        0
    }
}

unsafe fn get_image_size_in_inches(
    handle: rust_input_handle_t,
    width: *mut f64,
    height: *mut f64,
) -> c_int {
    unsafe {
        let mut err = 1;
        let mut width_pix: c_uint = 0;
        let mut height_pix: c_uint = 0;
        let mut xdensity: f64 = 0.0;
        let mut ydensity: f64 = 0.0;

        if check_for_jpeg(handle) != 0 {
            err = jpeg_get_bbox(
                handle,
                &mut width_pix,
                &mut height_pix,
                &mut xdensity,
                &mut ydensity,
            );
        } else if check_for_bmp(handle) != 0 {
            err = bmp_get_bbox(
                handle,
                &mut width_pix,
                &mut height_pix,
                &mut xdensity,
                &mut ydensity,
            );
        } else if check_for_png(handle) != 0 {
            err = png_get_bbox(
                handle,
                &mut width_pix,
                &mut height_pix,
                &mut xdensity,
                &mut ydensity,
            );
        }

        if err != 0 {
            *width = -1.0;
            *height = -1.0;
            return err;
        }

        *width = (width_pix as f64) * xdensity / 72.0;
        *height = (height_pix as f64) * ydensity / 72.0;
        0
    }
}

unsafe fn find_pic_file(
    path: *mut *mut c_char,
    bounds: *mut real_rect,
    pdfBoxType: c_int,
    page: c_int,
) -> c_int {
    unsafe {
        let handle =
            super::bridge::ttstub_input_open(name_of_file, super::bridge::TTBC_FILE_FORMAT_PICT, 0);
        (*bounds).x = 0.0;
        (*bounds).y = 0.0;
        (*bounds).wd = 0.0;
        (*bounds).ht = 0.0;

        if handle == super::bridge::INVALID_HANDLE {
            return 1;
        }

        let err = if pdfBoxType != 0 {
            pdf_get_rect(name_of_file, handle, page, pdfBoxType, bounds)
        } else {
            let mut wd: f64 = 0.0;
            let mut ht: f64 = 0.0;
            let ret = get_image_size_in_inches(handle, &mut wd, &mut ht);
            (*bounds).wd = (wd * 72.27) as f32;
            (*bounds).ht = (ht * 72.27) as f32;
            ret
        };

        if err == 0 {
            *path = libc::strdup(name_of_file);
        }

        super::bridge::ttstub_input_close(handle);
        err
    }
}

fn transform_point(p: &mut real_point, t: &transform_t) {
    let px = p.x as f64;
    let py = p.y as f64;
    p.x = (t.a * px + t.c * py + t.x) as f32;
    p.y = (t.b * px + t.d * py + t.y) as f32;
}

fn make_identity(t: &mut transform_t) {
    t.a = 1.0;
    t.b = 0.0;
    t.c = 0.0;
    t.d = 1.0;
    t.x = 0.0;
    t.y = 0.0;
}

fn make_scale(t: &mut transform_t, xscale: f64, yscale: f64) {
    t.a = xscale;
    t.b = 0.0;
    t.c = 0.0;
    t.d = yscale;
    t.x = 0.0;
    t.y = 0.0;
}

fn make_translation(t: &mut transform_t, dx: f64, dy: f64) {
    t.a = 1.0;
    t.b = 0.0;
    t.c = 0.0;
    t.d = 1.0;
    t.x = dx;
    t.y = dy;
}

fn make_rotation(t: &mut transform_t, a: f64) {
    t.a = a.cos();
    t.b = a.sin();
    t.c = -a.sin();
    t.d = a.cos();
    t.x = 0.0;
    t.y = 0.0;
}

fn transform_concat(t1: &mut transform_t, t2: &transform_t) {
    let r = transform_t {
        a: t1.a * t2.a + t1.b * t2.c + 0.0 * t2.x,
        b: t1.a * t2.b + t1.b * t2.d + 0.0 * t2.y,
        c: t1.c * t2.a + t1.d * t2.c + 0.0 * t2.x,
        d: t1.c * t2.b + t1.d * t2.d + 0.0 * t2.y,
        x: t1.x * t2.a + t1.y * t2.c + 1.0 * t2.x,
        y: t1.x * t2.b + t1.y * t2.d + 1.0 * t2.y,
    };
    *t1 = r;
}

#[unsafe(export_name = "load_picture")]
pub unsafe extern "C" fn load_picture(is_pdf: bool) {
    unsafe {
        tt_checkpoint_disallow();

        let mut pic_path: *mut c_char = ptr::null_mut();
        let mut bounds = real_rect::default();
        let mut t = transform_t::default();
        let mut t2 = transform_t::default();
        let mut corners = [real_point::default(); 4];
        let mut x_size_req: f64 = 0.0;
        let mut y_size_req: f64 = 0.0;
        let mut xmin: f64;
        let mut xmax: f64;
        let mut ymin: f64;
        let mut ymax: f64;

        scan_file_name();
        pack_file_name(cur_name, cur_area, cur_ext);

        let mut pdf_box_type: i32 = 0;
        let mut page: i32 = 0;

        if is_pdf {
            if scan_keyword(b"page\0".as_ptr() as *const c_char) {
                scan_int();
                page = cur_val;
            }
            pdf_box_type = pdfbox_none;
            if scan_keyword(b"crop\0".as_ptr() as *const c_char) {
                pdf_box_type = pdfbox_crop;
            } else if scan_keyword(b"media\0".as_ptr() as *const c_char) {
                pdf_box_type = pdfbox_media;
            } else if scan_keyword(b"bleed\0".as_ptr() as *const c_char) {
                pdf_box_type = pdfbox_bleed;
            } else if scan_keyword(b"trim\0".as_ptr() as *const c_char) {
                pdf_box_type = pdfbox_trim;
            } else if scan_keyword(b"art\0".as_ptr() as *const c_char) {
                pdf_box_type = pdfbox_art;
            }
        }

        let result = if pdf_box_type == pdfbox_none {
            find_pic_file(&mut pic_path, &mut bounds, pdfbox_crop, page)
        } else {
            find_pic_file(&mut pic_path, &mut bounds, pdf_box_type, page)
        };

        corners[0].x = bounds.x;
        corners[0].y = bounds.y;
        corners[1].x = corners[0].x;
        corners[1].y = bounds.y + bounds.ht;
        corners[2].x = bounds.x + bounds.wd;
        corners[2].y = corners[1].y;
        corners[3].x = corners[2].x;
        corners[3].y = corners[0].y;

        x_size_req = 0.0;
        y_size_req = 0.0;
        make_identity(&mut t);

        let mut check_keywords = true;
        while check_keywords {
            if scan_keyword(b"scaled\0".as_ptr() as *const c_char) {
                scan_int();
                if x_size_req == 0.0 && y_size_req == 0.0 {
                    make_scale(
                        &mut t2,
                        (cur_val as f64) / 1000.0,
                        (cur_val as f64) / 1000.0,
                    );
                    for c in &mut corners {
                        transform_point(c, &t2);
                    }
                    transform_concat(&mut t, &t2);
                }
            } else if scan_keyword(b"xscaled\0".as_ptr() as *const c_char) {
                scan_int();
                if x_size_req == 0.0 && y_size_req == 0.0 {
                    make_scale(&mut t2, (cur_val as f64) / 1000.0, 1.0);
                    for c in &mut corners {
                        transform_point(c, &t2);
                    }
                    transform_concat(&mut t, &t2);
                }
            } else if scan_keyword(b"yscaled\0".as_ptr() as *const c_char) {
                scan_int();
                if x_size_req == 0.0 && y_size_req == 0.0 {
                    make_scale(&mut t2, 1.0, (cur_val as f64) / 1000.0);
                    for c in &mut corners {
                        transform_point(c, &t2);
                    }
                    transform_concat(&mut t, &t2);
                }
            } else if scan_keyword(b"width\0".as_ptr() as *const c_char) {
                scan_dimen(false, false, false);
                if cur_val <= 0 {
                    error_here_with_diagnostic(c"Improper image ".as_ptr());
                    print_cstr(b"size (\0".as_ptr() as *const c_char);
                    print_scaled(cur_val);
                    print_cstr(b"pt) will be ignored\0".as_ptr() as *const c_char);
                    capture_to_diagnostic(ptr::null_mut());
                    help_ptr = 2;
                    help_line[1] = b"I can't scale images to zero or negative sizes,\0".as_ptr()
                        as *const c_char;
                    help_line[0] = b"so I'm ignoring this.\0".as_ptr() as *const c_char;
                    error();
                } else {
                    x_size_req = Fix2D(cur_val);
                }
            } else if scan_keyword(b"height\0".as_ptr() as *const c_char) {
                scan_dimen(false, false, false);
                if cur_val <= 0 {
                    error_here_with_diagnostic(c"Improper image ".as_ptr());
                    print_cstr(b"size (\0".as_ptr() as *const c_char);
                    print_scaled(cur_val);
                    print_cstr(b"pt) will be ignored\0".as_ptr() as *const c_char);
                    capture_to_diagnostic(ptr::null_mut());
                    help_ptr = 2;
                    help_line[1] = b"I can't scale images to zero or negative sizes,\0".as_ptr()
                        as *const c_char;
                    help_line[0] = b"so I'm ignoring this.\0".as_ptr() as *const c_char;
                    error();
                } else {
                    y_size_req = Fix2D(cur_val);
                }
            } else if scan_keyword(b"rotated\0".as_ptr() as *const c_char) {
                scan_decimal();
                if x_size_req != 0.0 || y_size_req != 0.0 {
                    xmin = 1000000.0;
                    xmax = -1000000.0;
                    ymin = xmin;
                    ymax = xmax;
                    for c in &corners {
                        let cx = c.x as f64;
                        let cy = c.y as f64;
                        if cx < xmin {
                            xmin = cx;
                        }
                        if cx > xmax {
                            xmax = cx;
                        }
                        if cy < ymin {
                            ymin = cy;
                        }
                        if cy > ymax {
                            ymax = cy;
                        }
                    }
                    if x_size_req == 0.0 {
                        make_scale(
                            &mut t2,
                            y_size_req / (ymax - ymin),
                            y_size_req / (ymax - ymin),
                        );
                    } else if y_size_req == 0.0 {
                        make_scale(
                            &mut t2,
                            x_size_req / (xmax - xmin),
                            x_size_req / (xmax - xmin),
                        );
                    } else {
                        make_scale(
                            &mut t2,
                            x_size_req / (xmax - xmin),
                            y_size_req / (ymax - ymin),
                        );
                    }
                    for c in &mut corners {
                        transform_point(c, &t2);
                    }
                    x_size_req = 0.0;
                    y_size_req = 0.0;
                    transform_concat(&mut t, &t2);
                }
                make_rotation(&mut t2, Fix2D(cur_val) * PI / 180.0);
                for c in &mut corners {
                    transform_point(c, &t2);
                }
                xmin = 1000000.0;
                xmax = -1000000.0;
                ymin = xmin;
                ymax = xmax;
                for c in &corners {
                    let cx = c.x as f64;
                    let cy = c.y as f64;
                    if cx < xmin {
                        xmin = cx;
                    }
                    if cx > xmax {
                        xmax = cx;
                    }
                    if cy < ymin {
                        ymin = cy;
                    }
                    if cy > ymax {
                        ymax = cy;
                    }
                }
                corners[0].x = xmin as f32;
                corners[0].y = ymin as f32;
                corners[1].x = xmin as f32;
                corners[1].y = ymax as f32;
                corners[2].x = xmax as f32;
                corners[2].y = ymax as f32;
                corners[3].x = xmax as f32;
                corners[3].y = ymin as f32;
                transform_concat(&mut t, &t2);
            } else {
                check_keywords = false;
            }
        }

        if x_size_req != 0.0 || y_size_req != 0.0 {
            xmin = 1000000.0;
            xmax = -1000000.0;
            ymin = xmin;
            ymax = xmax;
            for c in &corners {
                let cx = c.x as f64;
                let cy = c.y as f64;
                if cx < xmin {
                    xmin = cx;
                }
                if cx > xmax {
                    xmax = cx;
                }
                if cy < ymin {
                    ymin = cy;
                }
                if cy > ymax {
                    ymax = cy;
                }
            }
            if x_size_req == 0.0 {
                make_scale(
                    &mut t2,
                    y_size_req / (ymax - ymin),
                    y_size_req / (ymax - ymin),
                );
            } else if y_size_req == 0.0 {
                make_scale(
                    &mut t2,
                    x_size_req / (xmax - xmin),
                    x_size_req / (xmax - xmin),
                );
            } else {
                make_scale(
                    &mut t2,
                    x_size_req / (xmax - xmin),
                    y_size_req / (ymax - ymin),
                );
            }
            for c in &mut corners {
                transform_point(c, &t2);
            }
            x_size_req = 0.0;
            y_size_req = 0.0;
            transform_concat(&mut t, &t2);
        }

        xmin = 1000000.0;
        xmax = -1000000.0;
        ymin = xmin;
        ymax = xmax;
        for c in &corners {
            let cx = c.x as f64;
            let cy = c.y as f64;
            if cx < xmin {
                xmin = cx;
            }
            if cx > xmax {
                xmax = cx;
            }
            if cy < ymin {
                ymin = cy;
            }
            if cy > ymax {
                ymax = cy;
            }
        }

        make_translation(
            &mut t2,
            -(xmin as i32 as f64) * 72.0 / 72.27,
            -(ymin as i32 as f64) * 72.0 / 72.27,
        );
        transform_concat(&mut t, &t2);

        if result == 0 {
            let path_len = libc::strlen(pic_path);
            let mem_word_size = std::mem::size_of::<memory_word>();
            new_whatsit(
                (PIC_NODE) as i16,
                ((PIC_NODE_SIZE as usize + (path_len + mem_word_size - 1) / mem_word_size) as i32)
                    as i16,
            );

            let tail = cur_list.tail as usize;
            if is_pdf {
                (*mem.add(tail)).b16.s0 = PDF_NODE as u16;
            }
            (*mem.add(tail + 4)).b16.s1 = path_len as u16;
            (*mem.add(tail + 4)).b16.s0 = page as u16;
            (*mem.add(tail + 8)).b16.s1 = pdf_box_type as u16;
            (*mem.add(tail + 1)).b32.s1 = D2Fix(xmax - xmin);
            (*mem.add(tail + 3)).b32.s1 = D2Fix(ymax - ymin);
            (*mem.add(tail + 2)).b32.s1 = 0;
            (*mem.add(tail + 5)).b32.s0 = D2Fix(t.a);
            (*mem.add(tail + 5)).b32.s1 = D2Fix(t.b);
            (*mem.add(tail + 6)).b32.s0 = D2Fix(t.c);
            (*mem.add(tail + 6)).b32.s1 = D2Fix(t.d);
            (*mem.add(tail + 7)).b32.s0 = D2Fix(t.x);
            (*mem.add(tail + 7)).b32.s1 = D2Fix(t.y);

            let dest = mem.add(tail + PIC_NODE_SIZE as usize) as *mut u8;
            ptr::copy_nonoverlapping(pic_path as *const u8, dest, path_len);
            libc::free(pic_path as *mut c_void);
        } else {
            error_here_with_diagnostic(c"Unable to load picture or PDF file '".as_ptr());
            print_file_name(cur_name, cur_area, cur_ext);
            print(b'\'' as i32);
            capture_to_diagnostic(ptr::null_mut());
            if result == -43 {
                help_ptr = 2;
                help_line[1] =
                    b"The requested image couldn't be read because\0".as_ptr() as *const c_char;
                help_line[0] = b"the file was not found.\0".as_ptr() as *const c_char;
            } else {
                help_ptr = 2;
                help_line[1] =
                    b"The requested image couldn't be read because\0".as_ptr() as *const c_char;
                help_line[0] = b"it was not a recognized image format.\0".as_ptr() as *const c_char;
            }
            error();
        }
    }
}

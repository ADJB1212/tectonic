//! Canonical renderer representations shared with the C compatibility API.
#![allow(non_camel_case_types, clippy::too_many_arguments)]

use core::ffi::{c_char, c_double, c_int, c_uchar, c_void};

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_page_begin(unit: f64, precision: i32, autorotate: i32, direction: i32) -> i32 {
    crate::pipeline::page_begin(unit, precision, autorotate, direction)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_page_text(
    font: *const tt_pdf_render_font,
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    data: *const u8,
    length: usize,
) {
    unsafe {
        crate::pipeline::page_text(
            &*font,
            id,
            x,
            y,
            width,
            core::slice::from_raw_parts(data, length),
        );
    }
}

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_page_command(kind: i32, x: i32, y: i32, width: i32, height: i32) {
    crate::pipeline::page_command(kind, x, y, width, height);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_render_output(output: *mut c_void, data: *const c_char, length: usize) {
    unsafe {
        append_output(output, data, length);
    }
}

// SAFETY: output is an exclusively borrowed Vec and data is readable for length bytes.
pub(crate) unsafe fn append_output(output: *mut c_void, data: *const c_char, length: usize) {
    unsafe {
        let output = &mut *output.cast::<Vec<u8>>();
        output.extend_from_slice(core::slice::from_raw_parts(data.cast(), length));
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_tmatrix {
    pub a: c_double,
    pub b: c_double,
    pub c: c_double,
    pub d: c_double,
    pub e: c_double,
    pub f: c_double,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: c_double,
    pub lly: c_double,
    pub urx: c_double,
    pub ury: c_double,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_coord {
    pub x: c_double,
    pub y: c_double,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_info {
    pub width: c_double,
    pub height: c_double,
    pub depth: c_double,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: c_int,
}

/// Pointer-free font snapshot, copied when recording a page.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_pdf_render_font {
    pub name: [c_char; 16],
    pub size: i32,
    pub format: c_int,
    pub wmode: c_int,
    pub extend: c_double,
    pub slant: c_double,
    pub bold: c_double,
}

/// Command data is borrowed only for the duration of a render call.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_pdf_render_command {
    pub kind: c_int,
    pub font: c_int,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub data: *const c_uchar,
    pub length: usize,
}

#[unsafe(export_name = "__pdf_impl_tt_pdf_render_page")]
unsafe extern "C-unwind" fn tt_pdf_render_page(
    commands: *const tt_pdf_render_command,
    count: usize,
    fonts: *const tt_pdf_render_font,
    font_count: usize,
    unit: c_double,
    precision: c_int,
    autorotate: c_int,
    direction: c_int,
    output: *mut c_void,
) -> c_int {
    use crate::error::{PdfError, PdfErrorKind};
    use crate::renderer::{Direction, PageRenderer, RenderCommand, RenderSettings, TextRun};
    let render = || -> Result<(), PdfError> {
        unsafe {
            let commands = if count == 0 {
                &[]
            } else {
                core::slice::from_raw_parts(commands, count)
            };
            let fonts = if font_count == 0 {
                &[]
            } else {
                core::slice::from_raw_parts(fonts, font_count)
            };
            let output = output.cast::<Vec<u8>>().as_mut().ok_or_else(|| {
                PdfError::new(
                    PdfErrorKind::InvalidState,
                    c"Missing renderer output".to_owned(),
                )
            })?;
            let mut renderer = PageRenderer::new(
                fonts,
                RenderSettings {
                    unit,
                    precision,
                    autorotate: autorotate != 0,
                    direction: Direction::try_from(direction)?,
                },
                output,
            )?;
            for command in commands {
                let data = if command.length == 0 {
                    &[]
                } else {
                    core::slice::from_raw_parts(command.data, command.length)
                };
                renderer.command(match command.kind {
                    0 => RenderCommand::Bytes(data),
                    1 => RenderCommand::Text(TextRun {
                        font: usize::try_from(command.font).map_err(|_| {
                            PdfError::new(
                                PdfErrorKind::MalformedInput,
                                c"Invalid renderer font".to_owned(),
                            )
                        })?,
                        x: command.x,
                        y: command.y,
                        width: command.width,
                        data,
                    }),
                    2 => RenderCommand::Rule {
                        x: command.x,
                        y: command.y,
                        width: command.width,
                        height: command.height,
                    },
                    3 => RenderCommand::Graphics,
                    4 => RenderCommand::Direction(Direction::try_from(command.x)?),
                    5 => RenderCommand::ResetFonts {
                        new_page: command.x != 0,
                    },
                    _ => {
                        return Err(PdfError::new(
                            PdfErrorKind::Unsupported,
                            c"Unsupported renderer command".to_owned(),
                        ));
                    }
                })?;
            }
            Ok(())
        }
    };
    render().map_or(-1, |()| 0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_pdf_render_page(
    commands: *const tt_pdf_render_command,
    count: usize,
    fonts: *const tt_pdf_render_font,
    font_count: usize,
    unit: c_double,
    precision: c_int,
    autorotate: c_int,
    direction: c_int,
    output: *mut c_void,
    _result: *mut c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        tt_pdf_render_page(
            commands, count, fonts, font_count, unit, precision, autorotate, direction, output,
        )
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_sprint_matrix(
    buf: *mut c_char,
    matrix: *const pdf_tmatrix,
    _result: *mut c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::pdf_sprint_matrix(buf, matrix)
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_sprint_rect(
    buf: *mut c_char,
    rect: *const pdf_rect,
    _result: *mut c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::pdf_sprint_rect(buf, rect)
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_sprint_coord(
    buf: *mut c_char,
    c: *const pdf_coord,
    _result: *mut c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::pdf_sprint_coord(buf, c)
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_sprint_length(
    buf: *mut c_char,
    value: c_double,
    _result: *mut c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::pdf_sprint_length(buf, value)
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_sprint_number(
    buf: *mut c_char,
    value: c_double,
    _result: *mut c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::pdf_sprint_number(buf, value)
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_transform_info_clear(info: *mut transform_info) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::transform_info_clear(info)
    }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dev_unit_dviunit(_result: *mut c_double) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::dev_unit_dviunit()
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_get_param(
    param_type: c_int,
    _result: *mut c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::pdf_dev_get_param(param_type)
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_set_param(param_type: c_int, value: c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::pdf_dev_set_param(param_type, value)
    }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_get_dirmode(_result: *mut c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::pdf_dev_get_dirmode()
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_set_dirmode(text_dir: c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::pdf_dev_set_dirmode(text_dir)
    }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_set_rect(
    rect: *mut pdf_rect,
    x_user: i32,
    y_user: i32,
    width: i32,
    height: i32,
    depth: i32,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdev::pdf_dev_set_rect(rect, x_user, y_user, width, height, depth)
    }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_transform(
    p: *mut pdf_coord,
    matrix: *const pdf_tmatrix,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        crate::backend::dpx_pdfdraw::pdf_dev_transform(p, matrix)
    }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    unsafe extern "C" {
        fn tt_pdf_renderer_abi_layout(length: *mut usize) -> *const usize;
    }

    #[test]
    fn renderer_layout_matches_published_c_headers() {
        let expected = [
            size_of::<c_int>(),
            align_of::<c_int>(),
            size_of::<i32>(),
            align_of::<i32>(),
            size_of::<usize>(),
            align_of::<usize>(),
            size_of::<c_double>(),
            align_of::<c_double>(),
            usize::from(c_char::MIN < 0),
            size_of::<pdf_tmatrix>(),
            align_of::<pdf_tmatrix>(),
            offset_of!(pdf_tmatrix, a),
            offset_of!(pdf_tmatrix, b),
            offset_of!(pdf_tmatrix, c),
            offset_of!(pdf_tmatrix, d),
            offset_of!(pdf_tmatrix, e),
            offset_of!(pdf_tmatrix, f),
            size_of::<pdf_rect>(),
            align_of::<pdf_rect>(),
            offset_of!(pdf_rect, llx),
            offset_of!(pdf_rect, lly),
            offset_of!(pdf_rect, urx),
            offset_of!(pdf_rect, ury),
            size_of::<pdf_coord>(),
            align_of::<pdf_coord>(),
            offset_of!(pdf_coord, x),
            offset_of!(pdf_coord, y),
            size_of::<transform_info>(),
            align_of::<transform_info>(),
            offset_of!(transform_info, width),
            offset_of!(transform_info, height),
            offset_of!(transform_info, depth),
            offset_of!(transform_info, matrix),
            offset_of!(transform_info, bbox),
            offset_of!(transform_info, flags),
            size_of::<tt_pdf_render_font>(),
            align_of::<tt_pdf_render_font>(),
            offset_of!(tt_pdf_render_font, name),
            offset_of!(tt_pdf_render_font, size),
            offset_of!(tt_pdf_render_font, format),
            offset_of!(tt_pdf_render_font, wmode),
            offset_of!(tt_pdf_render_font, extend),
            offset_of!(tt_pdf_render_font, slant),
            offset_of!(tt_pdf_render_font, bold),
            size_of::<tt_pdf_render_command>(),
            align_of::<tt_pdf_render_command>(),
            offset_of!(tt_pdf_render_command, kind),
            offset_of!(tt_pdf_render_command, font),
            offset_of!(tt_pdf_render_command, x),
            offset_of!(tt_pdf_render_command, y),
            offset_of!(tt_pdf_render_command, width),
            offset_of!(tt_pdf_render_command, height),
            offset_of!(tt_pdf_render_command, data),
            offset_of!(tt_pdf_render_command, length),
        ];
        let mut length = 0;
        // SAFETY: The C helper returns an immutable static array of the reported length.
        let actual = unsafe {
            let layout = tt_pdf_renderer_abi_layout(&mut length);
            core::slice::from_raw_parts(layout, length)
        };
        assert_eq!(actual, expected);
    }
}

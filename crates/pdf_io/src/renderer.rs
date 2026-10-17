//! Borrowed renderer inputs. Plans retain ownership until rendering completes.

use crate::compat::renderer::tt_pdf_render_command;
pub(crate) use crate::compat::renderer::tt_pdf_render_font as RenderFont;
use crate::{
    diagnostics::format_message,
    error::{PdfError, PdfErrorKind},
};

pub(crate) struct RenderCommand<'a> {
    pub kind: i32,
    pub font: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub data: &'a [u8],
}

#[derive(Clone, Copy)]
pub(crate) struct RenderSettings {
    pub unit: f64,
    pub precision: i32,
    pub autorotate: i32,
    pub direction: i32,
}

pub(crate) fn render<'a>(
    commands: impl IntoIterator<Item = RenderCommand<'a>>,
    fonts: &[RenderFont],
    settings: RenderSettings,
) -> Result<Vec<u8>, PdfError> {
    if !(0..crate::numbers::POWERS_OF_TEN.len() as i32).contains(&settings.precision) {
        return Err(PdfError::new(
            PdfErrorKind::InvalidState,
            format_message(format_args!(
                "Invalid renderer precision {}",
                settings.precision
            )),
        ));
    }
    if fonts.len() > i32::MAX as usize || fonts.iter().any(|font| !font.name.contains(&0)) {
        return Err(PdfError::new(
            PdfErrorKind::MalformedInput,
            c"Invalid renderer font snapshot".to_owned(),
        ));
    }
    let commands = commands.into_iter();
    let mut native_commands = Vec::with_capacity(commands.size_hint().0);
    for command in commands {
        if !(0..=5).contains(&command.kind) {
            return Err(PdfError::new(
                PdfErrorKind::Unsupported,
                format_message(format_args!(
                    "Unsupported renderer command {}",
                    command.kind
                )),
            ));
        }
        native_commands.push(tt_pdf_render_command {
            kind: command.kind,
            font: command.font,
            x: command.x,
            y: command.y,
            width: command.width,
            height: command.height,
            data: command.data.as_ptr(),
            length: command.data.len(),
        });
    }
    let mut output = Vec::<u8>::new();
    // SAFETY: Inputs and output outlive the call. Font names are terminated,
    // precision is in range, and the backend checks command lengths and font IDs.
    let status = crate::backend::runtime::catch_backend(|| unsafe {
        Ok(crate::backend::render_page(
            native_commands.as_ptr(),
            native_commands.len(),
            fonts.as_ptr(),
            fonts.len(),
            settings.unit,
            settings.precision,
            settings.autorotate,
            settings.direction,
            (&mut output as *mut Vec<u8>).cast(),
        ))
    })?;
    if status != 0 {
        return Err(PdfError::new(
            PdfErrorKind::MalformedInput,
            c"Invalid renderer command".to_owned(),
        ));
    }
    output.push(b'\n');
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> RenderSettings {
        RenderSettings {
            unit: 1.0,
            precision: 3,
            autorotate: 0,
            direction: 0,
        }
    }

    #[test]
    fn borrowed_commands_preserve_raw_bytes_and_c_output() {
        use crate::compat::renderer::tt_pdf_render_command;
        use core::ffi::c_void;

        unsafe extern "C" {
            fn tt_pdf_render_page(
                commands: *const tt_pdf_render_command,
                count: usize,
                fonts: *const RenderFont,
                font_count: usize,
                unit: f64,
                precision: i32,
                autorotate: i32,
                direction: i32,
                output: *mut c_void,
            ) -> i32;
        }

        let data = b"q\n% \xff\nQ";
        let commands = [RenderCommand {
            kind: 0,
            font: 0,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            data,
        }];
        let rust = render(commands, &[], settings()).unwrap();
        assert_eq!(rust, b"q\n% \xff\nQ\n");
        let native = [tt_pdf_render_command {
            kind: 0,
            font: 0,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            data: data.as_ptr(),
            length: data.len(),
        }];
        let mut output = Vec::<u8>::new();
        // SAFETY: Inputs remain live and output is exclusively borrowed for the call.
        assert_eq!(
            unsafe {
                tt_pdf_render_page(
                    native.as_ptr(),
                    1,
                    [].as_ptr(),
                    0,
                    1.0,
                    3,
                    0,
                    0,
                    (&mut output as *mut Vec<u8>).cast(),
                )
            },
            0
        );
        output.push(b'\n');
        assert_eq!(rust, output);
    }

    #[test]
    fn rejected_commands_and_precision_allow_subsequent_rendering() {
        for (kind, font) in [(1, -1), (1, 0), (6, 0)] {
            let commands = [RenderCommand {
                kind,
                font,
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                data: b"",
            }];
            assert!(render(commands, &[], settings()).is_err());
        }
        for precision in [-1, 10, i32::MAX] {
            assert!(
                render(
                    [],
                    &[],
                    RenderSettings {
                        precision,
                        ..settings()
                    }
                )
                .is_err()
            );
        }
        let font = RenderFont {
            name: [1; 16],
            size: 10,
            format: 0,
            wmode: 0,
            extend: 1.0,
            slant: 0.0,
            bold: 0.0,
        };
        assert!(render([], &[font], settings()).is_err());
        assert_eq!(render([], &[], settings()).unwrap(), vec![b'\n']);
    }
}

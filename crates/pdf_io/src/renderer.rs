// Copyright 1998, 1999 Mark A. Wicks
// Copyright 2002-2019 Jin-Hwan Cho, Shunsaku Hirata, and the dvipdfmx team
// SPDX-License-Identifier: GPL-2.0-or-later

//! Page rendering from borrowed commands, with no coordinator or bridge access.

pub(crate) use crate::compat::renderer::tt_pdf_render_font as RenderFont;
use crate::error::{PdfError, PdfErrorKind};
use crate::numbers::{
    DECIMAL_BUFFER_SIZE, INVERSE_POWERS_OF_TEN, POWERS_OF_TEN, format_decimal, format_integer,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Direction {
    Horizontal,
    Vertical,
    ReverseVertical,
}

impl TryFrom<i32> for Direction {
    type Error = PdfError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Horizontal),
            1 => Ok(Self::Vertical),
            3 => Ok(Self::ReverseVertical),
            _ => Err(invalid(c"Invalid renderer direction")),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct TextRun<'a> {
    pub font: usize,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub data: &'a [u8],
}

#[derive(Clone, Copy)]
pub(crate) enum RenderCommand<'a> {
    Bytes(&'a [u8]),
    Text(TextRun<'a>),
    Rule {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    Graphics,
    Direction(Direction),
    ResetFonts {
        new_page: bool,
    },
}

#[derive(Clone, Copy)]
pub(crate) struct RenderSettings {
    pub unit: f64,
    pub precision: i32,
    pub autorotate: bool,
    pub direction: Direction,
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Graphics,
    Text,
    String,
}

#[derive(Clone, Copy, PartialEq)]
enum Rotation {
    Hh,
    Hv,
    Vh,
    Vv,
    Hd,
    Vd,
}

impl Rotation {
    fn new(vertical: bool, direction: Direction) -> Self {
        match (vertical, direction) {
            (false, Direction::Horizontal) => Self::Hh,
            (false, Direction::Vertical) => Self::Hv,
            (false, Direction::ReverseVertical) => Self::Hd,
            (true, Direction::Horizontal) => Self::Vh,
            (true, Direction::Vertical) => Self::Vv,
            (true, Direction::ReverseVertical) => Self::Vd,
        }
    }

    fn equivalent(self, other: Self) -> bool {
        self == other || matches!((self, other), (Self::Hh, Self::Vv) | (Self::Vv, Self::Hh))
    }
}

fn invalid(message: &std::ffi::CStr) -> PdfError {
    PdfError::new(PdfErrorKind::MalformedInput, message.to_owned())
}

pub(crate) struct PageRenderer<'a> {
    fonts: &'a [RenderFont],
    output: &'a mut Vec<u8>,
    settings: RenderSettings,
    mode: Mode,
    font: Option<usize>,
    offset: i32,
    reference: [i32; 2],
    slant: f64,
    extend: f64,
    rotation: Rotation,
    bold: f64,
    direction: Direction,
    force_reset: bool,
    multibyte: bool,
    minimum: i32,
}

impl<'a> PageRenderer<'a> {
    pub(crate) fn new(
        fonts: &'a [RenderFont],
        settings: RenderSettings,
        output: &'a mut Vec<u8>,
    ) -> Result<Self, PdfError> {
        let precision = usize::try_from(settings.precision)
            .ok()
            .filter(|&precision| precision < POWERS_OF_TEN.len())
            .ok_or_else(|| {
                PdfError::new(
                    PdfErrorKind::InvalidState,
                    c"Invalid renderer precision".to_owned(),
                )
            })?;
        if fonts.iter().any(|font| !font.name.contains(&0)) {
            return Err(invalid(c"Invalid renderer font snapshot"));
        }
        Ok(Self {
            fonts,
            output,
            settings,
            mode: Mode::Graphics,
            font: None,
            offset: 0,
            reference: [0; 2],
            slant: 0.0,
            extend: 1.0,
            rotation: Rotation::Hh,
            bold: 0.0,
            direction: settings.direction,
            force_reset: false,
            multibyte: false,
            minimum: (1.0 / (f64::from(POWERS_OF_TEN[precision]) * settings.unit) + 0.5).floor()
                as i32,
        })
    }

    fn number(&mut self, value: f64, precision: i32) -> Result<(), PdfError> {
        let mut bytes = [0; DECIMAL_BUFFER_SIZE];
        let length = format_decimal(value, precision, &mut bytes)
            .map_err(|_| invalid(c"Invalid PDF numeric precision"))?;
        self.output.extend_from_slice(&bytes[..length]);
        Ok(())
    }

    fn integer(&mut self, value: i32) {
        let mut bytes = [0; 11];
        let length = format_integer(value, &mut bytes).expect("integer buffer fits i32");
        self.output.extend_from_slice(&bytes[..length]);
    }

    fn position(&mut self, value: i32) -> Result<i32, PdfError> {
        let value = f64::from(value) * self.settings.unit;
        let step = INVERSE_POWERS_OF_TEN[self.settings.precision as usize];
        let error = value - (value / step + 0.5).floor() * step;
        self.number(value, self.settings.precision)?;
        Ok((error / self.settings.unit).round() as i32)
    }

    fn matrix(&mut self, x: i32, y: i32) -> Result<(), PdfError> {
        let values = match self.rotation {
            Rotation::Vh => [self.slant, 1.0, -self.extend, 0.0],
            Rotation::Hv => [0.0, -self.extend, 1.0, -self.slant],
            Rotation::Hh => [self.extend, 0.0, self.slant, 1.0],
            Rotation::Vv => [1.0, -self.slant, 0.0, self.extend],
            Rotation::Hd => [0.0, self.extend, -1.0, self.slant],
            Rotation::Vd => [-1.0, self.slant, 0.0, -self.extend],
        };
        for value in values {
            self.output.push(b' ');
            self.number(value, (self.settings.precision + 2).min(8))?;
        }
        self.output.push(b' ');
        self.number(
            f64::from(x) * self.settings.unit,
            self.settings.precision.max(2),
        )?;
        self.output.push(b' ');
        self.number(
            f64::from(y) * self.settings.unit,
            self.settings.precision.max(2),
        )?;
        self.output.extend_from_slice(b" Tm");
        self.reference = [x, y];
        Ok(())
    }

    fn reset_text(&mut self) -> Result<(), PdfError> {
        self.output.extend_from_slice(b" BT");
        if self.force_reset
            || self.slant != 0.0
            || self.extend != 1.0
            || !matches!(self.rotation, Rotation::Hh | Rotation::Vv)
        {
            self.matrix(0, 0)?;
        }
        self.reference = [0; 2];
        self.offset = 0;
        self.force_reset = false;
        Ok(())
    }

    fn close_string(&mut self) {
        self.output
            .extend_from_slice(if self.multibyte { b">]TJ" } else { b")]TJ" });
    }

    fn text_mode(&mut self) -> Result<(), PdfError> {
        match self.mode {
            Mode::String => self.close_string(),
            Mode::Graphics => self.reset_text()?,
            Mode::Text => {}
        }
        self.mode = Mode::Text;
        self.offset = 0;
        Ok(())
    }

    fn graphics(&mut self) {
        if self.mode == Mode::String {
            self.close_string();
        }
        if self.mode != Mode::Graphics {
            if self.bold != 0.0 {
                self.output.extend_from_slice(b" 0 Tr");
                self.bold = 0.0;
            }
            self.output.extend_from_slice(b" ET");
            self.force_reset = false;
            self.font = None;
        }
        self.mode = Mode::Graphics;
    }

    fn rotation_for(&self, vertical: bool, direction: Direction) -> Rotation {
        Rotation::new(
            vertical,
            if self.settings.autorotate {
                direction
            } else if vertical {
                Direction::Vertical
            } else {
                Direction::Horizontal
            },
        )
    }

    fn select_font(&mut self, id: usize, font: RenderFont) -> Result<(), PdfError> {
        self.text_mode()?;
        self.multibyte = font.format == 3;
        let rotation = self.rotation_for(font.wmode != 0, self.direction);
        if font.slant != self.slant
            || font.extend != self.extend
            || !rotation.equivalent(self.rotation)
        {
            self.force_reset = true;
        }
        self.slant = font.slant;
        self.extend = font.extend;
        self.rotation = rotation;
        self.output.extend_from_slice(b" /");
        for &byte in font.name.iter().take_while(|&&byte| byte != 0) {
            self.output.push(byte as u8);
        }
        self.output.push(b' ');
        self.number(
            f64::from(font.size) * self.settings.unit,
            (self.settings.precision + 1).min(8),
        )?;
        self.output.extend_from_slice(b" Tf");
        if font.bold > 0.0 || font.bold != self.bold {
            if font.bold <= 0.0 {
                self.output.extend_from_slice(b" 0 Tr");
            } else {
                use std::io::Write;
                self.output.extend_from_slice(b" 2 Tr ");
                if font.bold.is_nan() {
                    self.output
                        .extend_from_slice(if font.bold.is_sign_negative() {
                            b"-nan"
                        } else {
                            b"nan"
                        });
                } else {
                    write!(self.output, "{:.6}", font.bold).expect("Vec writes cannot fail");
                }
                self.output.extend_from_slice(b" w");
            }
        }
        self.bold = font.bold;
        self.font = Some(id);
        Ok(())
    }

    fn start_string(&mut self, x: i32, y: i32) -> Result<(), PdfError> {
        let dx = x.wrapping_sub(self.reference[0]);
        let dy = y.wrapping_sub(self.reference[1]);
        let (tx, ty) = match self.rotation {
            Rotation::Vh => (
                dy,
                (-(f64::from(dx) - f64::from(dy) * self.slant) / self.extend) as i32,
            ),
            Rotation::Hv => (
                (-(f64::from(dy) + f64::from(dx) * self.slant) / self.extend) as i32,
                dx,
            ),
            Rotation::Hh => (
                ((f64::from(dx) - f64::from(dy) * self.slant) / self.extend) as i32,
                dy,
            ),
            Rotation::Vv => (
                dx,
                ((f64::from(dy) + f64::from(dx) * self.slant) / self.extend) as i32,
            ),
            Rotation::Hd => (
                ((-(f64::from(dy) + f64::from(dx) * self.slant) / self.extend) as i32)
                    .wrapping_neg(),
                dx.wrapping_neg(),
            ),
            Rotation::Vd => (
                dx.wrapping_neg(),
                (((f64::from(dy) + f64::from(dx) * self.slant) / self.extend) as i32)
                    .wrapping_neg(),
            ),
        };
        self.output.push(b' ');
        let ex = self.position(tx)?;
        self.output.push(b' ');
        let ey = self.position(ty)?;
        let (ex, ey) = match self.rotation {
            Rotation::Vh => (ey.wrapping_neg(), ex),
            Rotation::Hv => (ey, ex.wrapping_neg()),
            Rotation::Hd => (ey.wrapping_neg(), ex.wrapping_neg()),
            Rotation::Vd => (ex.wrapping_neg(), ey.wrapping_neg()),
            _ => (ex, ey),
        };
        self.output
            .extend_from_slice(if self.multibyte { b" Td[<" } else { b" Td[(" });
        self.reference = [x.wrapping_sub(ex), y.wrapping_sub(ey)];
        self.offset = 0;
        Ok(())
    }

    fn string_mode(&mut self, x: i32, y: i32) -> Result<(), PdfError> {
        if self.mode == Mode::Graphics {
            self.reset_text()?;
        }
        if self.mode != Mode::String {
            if self.force_reset {
                self.matrix(x, y)?;
                self.output
                    .extend_from_slice(if self.multibyte { b"[<" } else { b"[(" });
                self.force_reset = false;
            } else {
                self.start_string(x, y)?;
            }
        }
        self.mode = Mode::String;
        Ok(())
    }

    fn text(&mut self, run: TextRun<'_>) -> Result<(), PdfError> {
        let font = *self
            .fonts
            .get(run.font)
            .ok_or_else(|| invalid(c"Invalid renderer font"))?;
        if font.format == 3 {
            if run.data.len() > 2048 || !run.data.len().is_multiple_of(2) {
                return Err(invalid(c"Invalid composite renderer text"));
            }
        } else {
            let mut escaped = 0;
            for &byte in run.data {
                if escaped > 4092 {
                    return Err(invalid(c"Renderer text exceeds buffer limit"));
                }
                escaped += if !(32..=126).contains(&byte) {
                    4
                } else if matches!(byte, b'(' | b')' | b'\\') {
                    2
                } else {
                    1
                };
            }
        }
        if self.font != Some(run.font) {
            self.select_font(run.font, font)?;
        }
        let (dx, dy) = match self.direction {
            Direction::Horizontal => (
                self.reference[0]
                    .wrapping_add(self.offset)
                    .wrapping_sub(run.x),
                run.y.wrapping_sub(self.reference[1]),
            ),
            Direction::Vertical => (
                run.y
                    .wrapping_sub(self.reference[1])
                    .wrapping_add(self.offset),
                run.x.wrapping_sub(self.reference[0]),
            ),
            Direction::ReverseVertical => (
                run.y
                    .wrapping_add(self.reference[1])
                    .wrapping_add(self.offset),
                run.x.wrapping_add(self.reference[0]),
            ),
        };
        let kern = if self.force_reset
            || i64::from(dy).abs() > i64::from(self.minimum)
            || i64::from(dx).abs() > i64::from((3.0 * font.extend * f64::from(font.size)) as i32)
        {
            self.text_mode()?;
            0
        } else {
            (1000.0 / font.extend * f64::from(dx) / f64::from(font.size)) as i32
        };
        if self.mode != Mode::String {
            self.string_mode(run.x, run.y)?;
        } else if kern != 0 {
            self.offset = self.offset.wrapping_sub(
                (f64::from(kern) * font.extend * (f64::from(font.size) / 1000.0)) as i32,
            );
            self.output.push(if self.multibyte { b'>' } else { b')' });
            self.integer(if font.wmode != 0 {
                kern.wrapping_neg()
            } else {
                kern
            });
            self.output.push(if self.multibyte { b'<' } else { b'(' });
        }
        if self.multibyte {
            let start = self.output.len();
            self.output.resize(start + run.data.len() * 2, 0);
            faster_hex::hex_encode(run.data, &mut self.output[start..]).unwrap();
        } else {
            for &byte in run.data {
                match byte {
                    32..=126 => {
                        if matches!(byte, b'(' | b')' | b'\\') {
                            self.output.push(b'\\');
                        }
                        self.output.push(byte);
                    }
                    _ => self.output.extend_from_slice(&[
                        b'\\',
                        b'0' + (byte >> 6),
                        b'0' + ((byte >> 3) & 7),
                        b'0' + (byte & 7),
                    ]),
                }
            }
        }
        self.offset = self.offset.wrapping_add(run.width);
        Ok(())
    }

    fn rule(&mut self, x: i32, y: i32, width: i32, height: i32) -> Result<(), PdfError> {
        self.graphics();
        self.output.extend_from_slice(b" q ");
        let thickness = f64::from(width.min(height)) * self.settings.unit;
        if thickness < 0.0 || thickness > 5.0 {
            for (index, value) in [x, y, width, height].into_iter().enumerate() {
                if index != 0 {
                    self.output.push(b' ');
                }
                self.number(
                    f64::from(value) * self.settings.unit,
                    self.settings.precision,
                )?;
            }
            self.output.extend_from_slice(b" re f");
        } else {
            let (stroke, points) = if width > height {
                let y = y.wrapping_add(height / 2);
                (height, [x, y, x.wrapping_add(width), y])
            } else {
                let x = x.wrapping_add(width / 2);
                (width, [x, y, x, y.wrapping_add(height)])
            };
            self.number(
                f64::from(stroke) * self.settings.unit,
                (self.settings.precision + 1).min(8),
            )?;
            self.output.extend_from_slice(b" w ");
            self.position(points[0])?;
            self.output.push(b' ');
            self.position(points[1])?;
            self.output.extend_from_slice(b" m ");
            self.position(points[2])?;
            self.output.push(b' ');
            self.position(points[3])?;
            self.output.extend_from_slice(b" l S");
        }
        self.output.extend_from_slice(b" Q");
        Ok(())
    }

    pub(crate) fn command(&mut self, command: RenderCommand<'_>) -> Result<(), PdfError> {
        match command {
            RenderCommand::Bytes(bytes) => self.output.extend_from_slice(bytes),
            RenderCommand::Text(run) => self.text(run)?,
            RenderCommand::Rule {
                x,
                y,
                width,
                height,
            } => self.rule(x, y, width, height)?,
            RenderCommand::Graphics => self.graphics(),
            RenderCommand::Direction(direction) => {
                let vertical = self.font.is_some_and(|font| self.fonts[font].wmode != 0);
                let rotation = self.rotation_for(vertical, direction);
                if self.font.is_some() && !rotation.equivalent(self.rotation) {
                    self.force_reset = true;
                }
                self.rotation = rotation;
                self.direction = direction;
            }
            RenderCommand::ResetFonts { new_page } => {
                self.font = None;
                self.slant = 0.0;
                self.extend = 1.0;
                self.rotation = Rotation::Hh;
                if new_page {
                    self.bold = 0.0;
                }
                self.multibyte = false;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) fn render<'a>(
    commands: impl IntoIterator<Item = RenderCommand<'a>>,
    fonts: &[RenderFont],
    settings: RenderSettings,
) -> Result<Vec<u8>, PdfError> {
    let mut output = Vec::new();
    let mut renderer = PageRenderer::new(fonts, settings, &mut output)?;
    for command in commands {
        renderer.command(command)?;
    }
    output.push(b'\n');
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compat::renderer::tt_pdf_render_command;

    #[test]
    fn renderer_preserves_positioning_escaping_rules_and_modes() {
        let mut fonts = [RenderFont {
            name: [0; 16],
            size: 655360,
            format: 1,
            wmode: 0,
            extend: 1.0,
            slant: 0.0,
            bold: 0.0,
        }; 4];
        for (index, font) in fonts.iter_mut().enumerate() {
            font.name[0] = b'F' as _;
            font.name[1] = (b'0' + index as u8) as _;
        }
        fonts[1].extend = 1.2;
        fonts[1].slant = -0.23;
        fonts[1].bold = 0.1234565;
        fonts[2].format = 3;
        fonts[2].wmode = 1;
        fonts[3].format = 3;
        fonts[3].slant = 0.3;
        let data: Vec<_> = (0..=255).collect();
        let command = |kind, font, x, y, width, height| tt_pdf_render_command {
            kind,
            font,
            x,
            y,
            width,
            height,
            data: data.as_ptr(),
            length: data.len(),
        };
        let mut snapshots = Vec::new();
        for precision in [0, 2, 5, 8] {
            for direction in [0, 1, 3] {
                for autorotate in [false, true] {
                    let mut native = vec![command(0, 0, 0, 0, 0, 0)];
                    for font in 0..4 {
                        native.push(command(1, font, 100003, -200009, 35001, 0));
                        native.push(command(1, font, 135012, -200009, 13001, 0));
                        native.push(command(1, font, 148057, -200003, 23001, 0));
                        native.push(command(4, 0, if direction == 0 { 1 } else { 3 }, 0, 0, 0));
                        native.push(command(1, font, 57013, 23007, 17001, 0));
                        native.push(command(3, 0, 0, 0, 0, 0));
                        native.push(command(5, 0, font % 2, 0, 0, 0));
                        native.push(command(4, 0, direction, 0, 0, 0));
                    }
                    for (width, height) in
                        [(500000, 10), (10, 500000), (500000, 700000), (-1234, 512)]
                    {
                        native.push(command(2, 0, -1134, 1723, width, height));
                    }
                    let commands = native.iter().map(|command| match command.kind {
                        0 => RenderCommand::Bytes(&data),
                        1 => RenderCommand::Text(TextRun {
                            font: command.font as usize,
                            x: command.x,
                            y: command.y,
                            width: command.width,
                            data: &data,
                        }),
                        2 => RenderCommand::Rule {
                            x: command.x,
                            y: command.y,
                            width: command.width,
                            height: command.height,
                        },
                        3 => RenderCommand::Graphics,
                        4 => RenderCommand::Direction(Direction::try_from(command.x).unwrap()),
                        5 => RenderCommand::ResetFonts {
                            new_page: command.x != 0,
                        },
                        _ => unreachable!(),
                    });
                    let rendered = render(
                        commands,
                        &fonts,
                        RenderSettings {
                            unit: 1.0 / 65781.76,
                            precision,
                            autorotate,
                            direction: Direction::try_from(direction).unwrap(),
                        },
                    )
                    .unwrap();
                    snapshots.push(rendered.iter().fold(0xcbf29ce484222325u64, |hash, byte| {
                        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
                    }));
                }
            }
        }
        assert_eq!(
            snapshots,
            [
                17119455239846451983,
                867347506718678585,
                14673347213661815377,
                17685668221806526997,
                1561836129714677070,
                11423007752299059030,
                15396184572959118015,
                8872427214895412126,
                2381189299245836186,
                15705540854468547059,
                2381189299245836186,
                9400032127631779230,
                5356109727324753460,
                17602311797589943438,
                13021514766415101788,
                16642916829899610313,
                13021514766415101788,
                4503346182880597448,
                2747736986524457124,
                6394770586432778611,
                3318184150999067278,
                17464659094605745618,
                3318184150999067278,
                12552094752186619687,
            ]
        );
    }
}

#[cfg(test)]
mod failure_tests {
    use super::*;

    #[test]
    fn invalid_commands_allow_reuse_of_caller_owned_output() {
        let mut output = Vec::new();
        let settings = RenderSettings {
            unit: 1.0,
            precision: 3,
            autorotate: true,
            direction: Direction::Horizontal,
        };
        let mut renderer = PageRenderer::new(&[], settings, &mut output).unwrap();
        let error = renderer
            .command(RenderCommand::Text(TextRun {
                font: 1,
                x: 0,
                y: 0,
                width: 0,
                data: b"bad",
            }))
            .unwrap_err();
        assert_eq!(error.kind, PdfErrorKind::MalformedInput);
        renderer.command(RenderCommand::Bytes(b"valid")).unwrap();
        assert_eq!(output, b"valid");
        for precision in [-1, 10, i32::MAX] {
            assert!(
                PageRenderer::new(
                    &[],
                    RenderSettings {
                        precision,
                        ..settings
                    },
                    &mut output
                )
                .is_err()
            );
        }
    }
}

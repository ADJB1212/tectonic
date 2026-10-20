use std::{
    ffi::{CStr, CString},
    fmt::{self, Write},
};

#[derive(Default)]
pub(crate) struct State {
    pub(crate) quiet: i32,
    pub(crate) last_warning: bool,
}

pub(crate) fn format_message(arguments: fmt::Arguments<'_>) -> CString {
    struct Buffer {
        bytes: [u8; 1024],
        length: usize,
    }

    impl Write for Buffer {
        fn write_str(&mut self, text: &str) -> fmt::Result {
            let length = text.len().min(1023 - self.length);
            self.bytes[self.length..self.length + length].copy_from_slice(&text.as_bytes()[..length]);
            self.length += length;
            Ok(())
        }
    }

    let mut buffer = Buffer { bytes: [0; 1024], length: 0 };
    buffer.write_fmt(arguments).expect("diagnostic buffer writes cannot fail");
    CStr::from_bytes_until_nul(&buffer.bytes).unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_formatting_preserves_byte_truncation_and_nul_handling() {
        let text = "a".repeat(1022);
        let message = format_message(format_args!("{text}é after"));
        assert_eq!(message.as_bytes().len(), 1023);
        assert_eq!(message.as_bytes()[1022], 0xc3);
        assert_eq!(format_message(format_args!("before\0after")).as_c_str(), c"before");
        assert_eq!(
            format_message(format_args!("Invalid index {} for dfont.", -3)).as_c_str(),
            c"Invalid index -3 for dfont."
        );
    }
}

//! Bounded byte-line reads shared by the native input adapters.

use std::mem::MaybeUninit;

pub(crate) fn read_line(buffer: &mut [MaybeUninit<u8>], mut get: impl FnMut() -> Option<u8>, mut unget: impl FnMut(u8)) -> Option<usize> {
    if buffer.is_empty() {
        return None;
    }
    let mut length = 0;
    while length < buffer.len() - 1 {
        let Some(byte) = get() else {
            buffer[length].write(0);
            return if length == 0 { None } else { Some(length) };
        };
        if byte == b'\r' || byte == b'\n' {
            if byte == b'\r'
                && let Some(next) = get()
                && next != b'\n'
            {
                unget(next);
            }
            break;
        }
        buffer[length].write(byte);
        length += 1;
    }
    buffer[length].write(0);
    Some(length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn handles_line_endings_eof_and_non_utf8_bytes() {
        for (input, expected, consumed) in [
            (&b"\nrest"[..], &b""[..], 1),
            (&b"abc\r\nrest"[..], &b"abc"[..], 5),
            (&b"abc\rrest"[..], &b"abc"[..], 4),
            (&b"abc\r"[..], &b"abc"[..], 4),
            (&b"\xff\0\x80"[..], &b"\xff\0\x80"[..], 3),
        ] {
            let offset = Cell::new(0);
            let mut bytes = [MaybeUninit::new(0xaa); 10];
            let length = read_line(
                &mut bytes,
                || {
                    let byte = input.get(offset.get()).copied();
                    if byte.is_some() {
                        offset.set(offset.get() + 1);
                    }
                    byte
                },
                |_| offset.set(offset.get() - 1),
            )
            .unwrap();
            let bytes = bytes.map(|byte| unsafe { byte.assume_init() });
            assert_eq!(&bytes[..length], expected);
            assert_eq!(bytes[length], 0);
            assert_eq!(offset.get(), consumed);
        }
        let mut bytes = [MaybeUninit::new(0xaa); 2];
        assert_eq!(read_line(&mut bytes, || None, |_| unreachable!()), None);
        let bytes = bytes.map(|byte| unsafe { byte.assume_init() });
        assert_eq!(bytes, [0, 0xaa]);
    }

    #[test]
    fn capacity_limits_do_not_consume_input() {
        assert_eq!(read_line(&mut [], || panic!("read"), |_| panic!("unread")), None);
        assert_eq!(read_line(&mut [MaybeUninit::new(0xaa)], || panic!("read"), |_| panic!("unread")), Some(0));
        let mut input = b"abc\n".iter().copied();
        let mut bytes = [MaybeUninit::new(0xaa); 3];
        assert_eq!(read_line(&mut bytes, || input.next(), |_| unreachable!()), Some(2));
        let bytes = bytes.map(|byte| unsafe { byte.assume_init() });
        assert_eq!(bytes, [b'a', b'b', 0]);
        assert_eq!(input.next(), Some(b'c'));
    }

    #[test]
    fn initializes_only_the_returned_line_and_terminator() {
        let mut bytes = [MaybeUninit::uninit(); 8];
        let mut input = b"abc\n".iter().copied();
        let length = read_line(&mut bytes, || input.next(), |_| unreachable!()).unwrap();
        assert_eq!(length, 3);
        let initialized: Vec<u8> = bytes[..=length].iter().map(|byte| unsafe { byte.assume_init() }).collect();
        assert_eq!(initialized, b"abc\0");
    }
}

use super::{dpx_dpxcrypt::*, runtime};
use std::{ffi::CStr, fmt::Write, mem::zeroed};

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut text, byte| {
            write!(text, "{byte:02x}").unwrap();
            text
        })
}

#[test]
fn digest_vectors_accept_unaligned_output() {
    unsafe {
        let mut output = [0u8; 72];
        let offset = (9 - output.as_ptr().addr() % 8) % 8;
        let result = output.as_mut_ptr().add(offset);
        let mut md5: MD5_CONTEXT = zeroed();
        MD5_init(&mut md5);
        MD5_write(&mut md5, b"a".as_ptr(), 1);
        MD5_write(&mut md5, b"bc".as_ptr(), 2);
        MD5_final(result, &mut md5);
        assert_eq!(
            hex(&output[offset..offset + 16]),
            "900150983cd24fb0d6963f7d28e17f72"
        );

        let mut sha256: SHA256_CONTEXT = zeroed();
        SHA256_init(&mut sha256);
        SHA256_write(&mut sha256, b"abc".as_ptr(), 3);
        SHA256_final(result, &mut sha256);
        assert_eq!(
            hex(&output[offset..offset + 32]),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );

        let mut sha512: SHA512_CONTEXT = zeroed();
        SHA512_init(&mut sha512);
        SHA512_write(&mut sha512, b"abc".as_ptr(), 3);
        SHA512_final(result, &mut sha512);
        assert_eq!(
            hex(&output[offset..offset + 64]),
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );

        SHA384_init(&mut sha512);
        SHA512_write(&mut sha512, b"abc".as_ptr(), 3);
        SHA512_final(result, &mut sha512);
        assert_eq!(
            hex(&output[offset..offset + 48]),
            "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"
        );
    }
}

#[test]
fn encryption_matches_known_vectors() {
    unsafe {
        let key: Vec<u8> = (0..32).collect();
        let plain: Vec<u8> = (0..16).map(|byte| byte * 17).collect();
        for (key_len, expected) in [
            (16, "69c4e0d86a7b0430d8cdb78070b4c55a"),
            (24, "dda97ca4864cdfe06eaf70a0ec0d7191"),
            (32, "8ea2b7ca516745bfeafc49904b496089"),
        ] {
            let mut cipher = std::ptr::null_mut();
            let mut length = 0;
            AES_ecb_encrypt(
                key.as_ptr(),
                key_len,
                plain.as_ptr(),
                plain.len(),
                &mut cipher,
                &mut length,
            );
            assert_eq!(hex(std::slice::from_raw_parts(cipher, length)), expected);
            libc::free(cipher.cast());
        }

        let mut context: ARC4_CONTEXT = zeroed();
        let mut cipher = [0; 9];
        ARC4_set_key(&mut context, 3, b"Key".as_ptr());
        ARC4(&mut context, 9, b"Plaintext".as_ptr(), cipher.as_mut_ptr());
        assert_eq!(hex(&cipher), "bbf316e8d940af0ad3");
    }
}

#[test]
fn backend_errors_unwind_destructors_before_returning_to_c() {
    struct Guard<'a>(&'a mut bool);
    impl Drop for Guard<'_> {
        fn drop(&mut self) {
            *self.0 = true;
        }
    }

    let mut dropped = false;
    assert!(
        runtime::boundary(|| {
            let _guard = Guard(&mut dropped);
            runtime::raise(c"invalid PDF input".to_owned());
        })
        .is_err()
    );
    assert!(dropped);
    assert_eq!(
        unsafe { CStr::from_ptr(runtime::tt_pdf_last_error()) },
        c"invalid PDF input"
    );
    assert_eq!(runtime::boundary(|| 42), Ok(42));
}

#[test]
fn unicode_legacy_encoding_and_decoding_are_preserved() {
    use super::dpx_unicode::*;
    unsafe {
        for (input, expected) in [
            (&b"\xc0\x80"[..], 0),
            (&b"\xf8\x80\x80\x80\x80"[..], 0),
            (&b"\xfc\x80\x80\x80\x80\x80"[..], 0),
        ] {
            let mut pointer = input.as_ptr();
            assert_eq!(
                UC_UTF8_decode_char(&mut pointer, pointer.add(input.len())),
                expected
            );
            assert_eq!(pointer, input.as_ptr().add(input.len()));
        }
        let input = [0xd8, 0, 0, 0];
        let mut pointer = input.as_ptr();
        assert_eq!(
            UC_UTF16BE_decode_char(&mut pointer, pointer.add(4)),
            0x10000
        );
        let mut output = [0; 4];
        let mut pointer = output.as_mut_ptr();
        assert_eq!(UC_UTF8_encode_char(0x7f, &mut pointer, pointer.add(4)), 2);
        assert_eq!(&output[..2], &[0xc1, 0xbf]);
        let mut pointer = output.as_mut_ptr();
        assert_eq!(
            UC_UTF16BE_encode_char(0xd800, &mut pointer, pointer.add(4)),
            2
        );
        assert_eq!(&output[..2], &[0xd8, 0]);
        let mut pointer = output.as_mut_ptr();
        assert_eq!(UC_UTF16BE_encode_char(-1, &mut pointer, pointer.add(4)), 2);
        assert_eq!(&output[..2], &[0xff, 0xfd]);
    }
}

#[test]
fn unicode_adapters_preserve_positions_and_output_on_failure() {
    use super::dpx_unicode::*;
    unsafe {
        for (decode, input) in [
            (
                UC_UTF8_decode_char as unsafe extern "C-unwind" fn(_, _) -> _,
                &b"\xe2\x82"[..],
            ),
            (UC_UTF16BE_decode_char, &b"\xd8\0\xdc"[..]),
        ] {
            for length in 0..=input.len() {
                let mut pointer = input.as_ptr();
                assert_eq!(decode(&mut pointer, input.as_ptr().add(length)), -1);
                assert_eq!(pointer, input.as_ptr());
            }
        }
        for encode in [UC_UTF8_encode_char, UC_UTF16BE_encode_char] {
            for capacity in 0..4 {
                let mut bytes = [0xaa; 6];
                let mut pointer = bytes.as_mut_ptr();
                let end = pointer.add(capacity);
                assert_eq!(encode(0x10000, &mut pointer, end), 0);
                assert_eq!(pointer, bytes.as_mut_ptr());
                assert_eq!(bytes, [0xaa; 6]);
            }
            let mut bytes = [0xaa; 6];
            let mut pointer = bytes.as_mut_ptr();
            let end = pointer.add(6);
            assert_eq!(encode(0x10000, &mut pointer, end), 4);
            assert_eq!(pointer, bytes.as_mut_ptr().add(4));
            assert_eq!(&bytes[4..], &[0xaa; 2]);
        }
    }
}

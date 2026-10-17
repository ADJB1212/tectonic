use super::{dpx_dpxcrypt::*, runtime};
use std::{ffi::CStr, fmt::Write, mem::zeroed};

#[test]
fn c_allocator_handoffs_remain_c_owned_across_document_completion() {
    let document = crate::compat::document::DocumentScope::new().unwrap();
    unsafe {
        let mut memory = std::ptr::null_mut();
        assert_eq!(super::dpx_mem::__pdf_entry_new(8, &mut memory), 0);
        memory.cast::<u8>().write_bytes(0x5a, 8);
        assert!((*crate::compat::document::current()).allocations.is_empty());
        assert_eq!(
            super::dpx_mem::__pdf_entry_renew(memory, 32, &mut memory),
            0
        );
        assert!((*crate::compat::document::current()).allocations.is_empty());
        let input = b"identifier ";
        let mut start = input.as_ptr().cast();
        let end = input.as_ptr().add(input.len()).cast();
        let mut identifier = std::ptr::null_mut();
        assert_eq!(
            super::dpx_pdfparse::__pdf_entry_parse_ident(&mut start, end, &mut identifier),
            0
        );
        assert!((*crate::compat::document::current()).allocations.is_empty());
        let mut cipher = std::ptr::null_mut();
        let mut cipher_len = 0;
        assert_eq!(
            __pdf_entry_AES_ecb_encrypt(
                [0u8; 16].as_ptr(),
                16,
                [0u8; 16].as_ptr(),
                16,
                &mut cipher,
                &mut cipher_len
            ),
            0
        );
        assert_eq!(cipher_len, 16);
        assert!((*crate::compat::document::current()).allocations.is_empty());
        drop(document);
        assert_eq!(
            std::slice::from_raw_parts(memory.cast::<u8>(), 8),
            &[0x5a; 8]
        );
        assert_eq!(CStr::from_ptr(identifier), c"identifier");
        assert_eq!(
            std::slice::from_raw_parts(cipher, cipher_len),
            &[
                0x66, 0xe9, 0x4b, 0xd4, 0xef, 0x8a, 0x2c, 0x3b, 0x88, 0x4c, 0xfa, 0x59, 0xca, 0x34,
                0x2b, 0x2e
            ]
        );
        libc::free(memory);
        libc::free(identifier.cast());
        libc::free(cipher.cast());
    }
}

#[test]
fn document_failure_reclaims_state_and_resets_abi_configuration_and_diagnostics() {
    use crate::{
        compat::{allocation, document},
        error::{PdfError, PdfErrorKind},
    };
    let failure = runtime::catch_backend::<()>(|| {
        let _document = document::DocumentScope::new()?;
        assert_eq!(
            document::DocumentScope::new().err().unwrap().kind,
            PdfErrorKind::InvalidState
        );
        document::with_diagnostics(|state| {
            state.quiet = 2;
            state.last_warning = true;
        });
        unsafe {
            super::dpx_dpxconf::dpx_conf.verbose_level = 7;
            super::dpx_dpxconf::dpx_conf.file.keep_cache = 1;
            super::dpx_cid::opt_flags_cidfont = 123;
            let memory = allocation::malloc(32);
            assert!(!memory.is_null());
            let memory = allocation::realloc(memory, 64);
            assert!(!memory.is_null());
            assert_eq!((*document::current()).allocations.len(), 1);
            allocation::free(memory);
            assert!((*document::current()).allocations.is_empty());
            let abandoned = allocation::malloc(16);
            assert!(!abandoned.is_null());
        }
        runtime::legacy_result(Err(PdfError::new(
            PdfErrorKind::MalformedInput,
            c"injected document failure".to_owned(),
        )))
    });
    assert_eq!(failure.unwrap_err().kind, PdfErrorKind::MalformedInput);
    assert!(!document::active());
    let _next = document::DocumentScope::new().unwrap();
    document::with_diagnostics(|state| {
        assert_eq!(state.quiet, 0);
        assert!(!state.last_warning);
    });
    unsafe {
        assert_eq!({ super::dpx_dpxconf::dpx_conf.verbose_level }, 0);
        assert_eq!({ super::dpx_dpxconf::dpx_conf.file.keep_cache }, 0);
        assert_eq!({ super::dpx_cid::opt_flags_cidfont }, 0);
        assert!((*document::current()).allocations.is_empty());
    }
}

#[test]
fn typed_errors_cross_only_the_compatibility_boundary() {
    use crate::error::{PdfError, PdfErrorKind};
    let error = runtime::catch_backend::<()>(|| {
        runtime::raise(c"legacy failure".to_owned());
    })
    .unwrap_err();
    assert_eq!(error.kind, PdfErrorKind::Backend);
    assert_eq!(error.message.as_c_str(), c"legacy failure");

    let dropped = std::cell::Cell::new(false);
    struct Guard<'a>(&'a std::cell::Cell<bool>);
    impl Drop for Guard<'_> {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }
    let error = runtime::catch_backend::<()>(|| {
        let _guard = Guard(&dropped);
        runtime::legacy_result(Err(PdfError::new(
            PdfErrorKind::MalformedInput,
            c"malformed".to_owned(),
        )
        .at(19)
        .in_file(b"bad\xff.tex")))
    })
    .unwrap_err();
    assert!(dropped.get());
    assert_eq!(error.kind, PdfErrorKind::MalformedInput);
    assert_eq!(error.offset, Some(19));
    assert_eq!(error.file.as_deref(), Some(&b"bad\xff.tex"[..]));
    assert!(runtime::result_boundary::<()>(|| Err(error)).is_err());
    assert_eq!(
        unsafe { CStr::from_ptr(runtime::tt_pdf_last_error()) },
        c"malformed"
    );
    assert_eq!(runtime::result_boundary(|| Ok(42)), Ok(42));

    let panic = std::panic::catch_unwind(|| {
        let _ = runtime::catch_backend::<()>(|| panic!("unexpected bug"));
    })
    .unwrap_err();
    assert_eq!(panic.downcast_ref::<&str>(), Some(&"unexpected bug"));
}

#[test]
fn rust_diagnostic_macros_match_legacy_formatting_and_truncation() {
    let rust = pdf_format!("Invalid index {} for dfont.", -3);
    let legacy = pdf_format!(c"Invalid index %d for dfont.".as_ptr(), -3);
    assert_eq!(rust, legacy);
    let rust = pdf_format!("{}", "a".repeat(1100));
    let input = std::ffi::CString::new("a".repeat(1100)).unwrap();
    let legacy = pdf_format!(c"%s".as_ptr(), input.as_ptr());
    assert_eq!(rust, legacy);
}

#[test]
fn stdio_errors_return_through_c_status_and_allow_another_operation() {
    struct File(*mut libc::FILE);
    impl Drop for File {
        fn drop(&mut self) {
            unsafe {
                libc::fclose(self.0);
            }
        }
    }
    unsafe {
        let file = File(libc::tmpfile());
        assert!(!file.0.is_null());
        assert_eq!(libc::fwrite(b"abc".as_ptr().cast(), 1, 3, file.0), 3);
        let mut size = -1;
        assert_eq!(
            super::dpx_mfileio::__pdf_entry_file_size(file.0.cast(), &mut size),
            0
        );
        assert_eq!(size, 3);
        assert_eq!(libc::ftell(file.0), 0);
        assert_eq!(
            super::dpx_mfileio::__pdf_entry_seek_relative(file.0.cast(), -1),
            -1
        );
        assert_eq!(
            CStr::from_ptr(runtime::tt_pdf_last_error()),
            c"io:  An OS command failed that should not have.\n"
        );
        assert_eq!(
            super::dpx_mfileio::__pdf_entry_file_size(file.0.cast(), &mut size),
            0
        );
        assert_eq!(size, 3);
    }
}

#[test]
fn sfnt_directory_initialization_can_be_cleaned_up_after_failure() {
    unsafe {
        let font = runtime::allocate(std::mem::size_of::<super::dpx_sfnt::sfnt>())
            .cast::<super::dpx_sfnt::sfnt>();
        font.write(super::dpx_sfnt::sfnt {
            type_0: 0,
            directory: std::ptr::null_mut(),
            handle: 0,
            offset: 0,
        });
        assert!(runtime::boundary(|| super::dpx_sfnt::sfnt_read_table_directory(font, 0)).is_err());
        assert_eq!(
            CStr::from_ptr(runtime::tt_pdf_last_error()),
            c"assertion failed: sfont->handle"
        );
        super::dpx_sfnt::sfnt_close(font);
    }
    assert_eq!(runtime::boundary(|| 42), Ok(42));
}

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

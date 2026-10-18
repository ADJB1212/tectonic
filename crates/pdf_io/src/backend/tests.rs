use super::{dpx_dpxcrypt::*, runtime};
use std::{ffi::CStr, fmt::Write, mem::zeroed};

#[test]
fn pdf_parser_checks_container_depth_truncated_tokens_and_recovers() {
    let _document = crate::compat::document::DocumentScope::new().unwrap();
    crate::compat::document::with_diagnostics(|state| state.quiet = 2);
    unsafe {
        let mut inputs = vec![
            b"<".to_vec(),
            b"<<".to_vec(),
            b"[true".to_vec(),
            b"(\\".to_vec(),
            b"<< /a 1 >".to_vec(),
        ];
        inputs.push([vec![b'['; 257], vec![b']'; 257]].concat());
        for bytes in inputs {
            let mut start = bytes.as_ptr().cast();
            let result = super::dpx_pdfparse::parse_pdf_object(
                &mut start,
                bytes.as_ptr().add(bytes.len()).cast(),
                core::ptr::null_mut(),
            );
            assert!(result.is_null());
            let bytes = b"<< /a [true null 12.25 (ok)] >>";
            let mut start = bytes.as_ptr().cast();
            let result = super::dpx_pdfparse::parse_pdf_object(
                &mut start,
                bytes.as_ptr().add(bytes.len()).cast(),
                core::ptr::null_mut(),
            );
            assert!(!result.is_null());
            assert_eq!(start, bytes.as_ptr().add(bytes.len()).cast());
            super::dpx_pdfobj::pdf_release_obj(result.cast());
        }
        let name = [b"/".as_slice(), &[b'a'; 200]].concat();
        let mut start = name.as_ptr().cast();
        let result =
            super::dpx_pdfparse::parse_pdf_name(&mut start, name.as_ptr().add(name.len()).cast());
        assert_eq!(
            CStr::from_ptr(super::dpx_pdfobj::pdf_name_value(result.cast())).to_bytes(),
            &[b'a'; 200]
        );
        super::dpx_pdfobj::pdf_release_obj(result.cast());
    }
}

#[test]
fn object_containers_preserve_shared_identity_replacement_and_order() {
    use super::dpx_pdfobj::*;
    let _document = crate::compat::document::DocumentScope::new().unwrap();
    unsafe {
        let shared = pdf_new_number(12.0);
        let array = pdf_new_array();
        pdf_add_array(array, shared);
        let dict = pdf_new_dict();
        pdf_add_dict(dict, pdf_new_name(c"a".as_ptr()), pdf_link_obj(shared));
        pdf_add_dict(dict, pdf_new_name(c"b".as_ptr()), array);
        assert_eq!(
            pdf_add_dict(dict, pdf_new_name(c"a".as_ptr()), pdf_new_number(42.0)),
            1
        );
        assert_eq!(pdf_number_value(pdf_get_array(array, -1)), 12.0);
        assert!(pdf_get_array(array, -2).is_null());
        let keys = pdf_dict_keys(dict);
        assert_eq!(CStr::from_ptr(pdf_name_value(pdf_get_array(keys, 0))), c"a");
        assert_eq!(CStr::from_ptr(pdf_name_value(pdf_get_array(keys, 1))), c"b");
        pdf_release_obj(keys);
        for _ in 0..3000 {
            pdf_add_array(array, pdf_new_null());
        }
        assert_eq!(pdf_get_array(array, 0), shared);
        assert_eq!(pdf_number_value(shared), 12.0);
        pdf_remove_dict(dict, c"a".as_ptr());
        pdf_add_dict(dict, pdf_new_name(c"a".as_ptr()), pdf_new_boolean(1));
        let keys = pdf_dict_keys(dict);
        assert_eq!(CStr::from_ptr(pdf_name_value(pdf_get_array(keys, 0))), c"b");
        assert_eq!(CStr::from_ptr(pdf_name_value(pdf_get_array(keys, 1))), c"a");
        pdf_release_obj(keys);
        pdf_release_obj(dict);
        assert_eq!(pdf_obj_typeof(shared), PDF_OBJ_INVALID);
        assert_eq!(pdf_obj_typeof(array), PDF_OBJ_INVALID);
        assert_ne!(pdf_new_null(), shared);
    }
}

#[test]
fn indirect_cycles_have_no_ownership_cycle_and_released_targets_are_detected() {
    use super::dpx_pdfobj::*;
    let _document = crate::compat::document::DocumentScope::new().unwrap();
    unsafe {
        let first = pdf_new_dict();
        let second = pdf_new_dict();
        let external = pdf_ref_obj(first);
        let next = pdf_ref_obj(second);
        pdf_add_dict(first, pdf_new_name(c"Next".as_ptr()), next);
        let back = pdf_ref_obj(first);
        pdf_add_dict(second, pdf_new_name(c"Prev".as_ptr()), back);
        let resolved = pdf_deref_obj(back);
        assert_eq!(resolved, first);
        pdf_release_obj(resolved);
        pdf_release_obj(first);
        assert!(pdf_deref_obj(external).is_null());
        assert_eq!(pdf_obj_typeof(first), PDF_OBJ_INVALID);
        assert_eq!(pdf_obj_typeof(second), PDF_DICT);
        pdf_release_obj(second);
        pdf_release_obj(external);
        assert_eq!(pdf_obj_typeof(back), PDF_OBJ_INVALID);
        assert_eq!(pdf_obj_typeof(next), PDF_OBJ_INVALID);
    }
}

#[test]
fn object_leases_preserve_queued_targets_until_the_last_owner_releases() {
    use super::dpx_pdfobj::*;
    let _document = crate::compat::document::DocumentScope::new().unwrap();
    unsafe {
        let contents = pdf_new_stream(0);
        pdf_add_stream(contents, b"pending".as_ptr().cast(), 7);
        let queued = pdf_link_obj(contents);
        pdf_release_obj(contents);
        assert_eq!(pdf_stream_length(queued), 7);
        assert_eq!(
            std::slice::from_raw_parts(pdf_stream_dataptr(queued).cast::<u8>(), 7),
            b"pending"
        );
        pdf_release_obj(queued);
        assert_eq!(pdf_obj_typeof(contents), PDF_OBJ_INVALID);
    }
}

#[test]
fn name_tree_sorting_preserves_binary_keys_and_c_table_queries() {
    use super::{dpx_pdfnames::*, dpx_pdfobj::*};
    let _document = crate::compat::document::DocumentScope::new().unwrap();
    unsafe {
        let mut names = pdf_new_name_tree();
        for (index, key) in [b"z".as_slice(), b"a\0b", b"a\xff", b"a"]
            .into_iter()
            .enumerate()
        {
            let number = pdf_new_number(index as f64);
            assert_eq!(
                pdf_names_add_object(names, key.as_ptr().cast(), key.len() as i32, number),
                0
            );
            assert_eq!(
                pdf_names_lookup_object(names, key.as_ptr().cast(), key.len() as i32),
                number
            );
        }
        assert_eq!((*names).count, 4);
        let mut count = 0;
        let tree = pdf_names_create_tree(names, &mut count, std::ptr::null_mut());
        assert_eq!(count, 4);
        let entries = pdf_lookup_dict(tree, c"Names".as_ptr());
        for (index, expected) in [b"a".as_slice(), b"a\0b", b"a\xff", b"z"]
            .into_iter()
            .enumerate()
        {
            let key = pdf_get_array(entries, index as i32 * 2);
            assert_eq!(
                std::slice::from_raw_parts(
                    pdf_string_value(key).cast::<u8>(),
                    pdf_string_length(key) as usize
                ),
                expected
            );
        }
        pdf_release_obj(tree);
        pdf_delete_name_tree(&mut names);
        assert!(names.is_null());
    }
}

#[test]
fn name_reservations_resolve_after_definition_without_recycling_placeholder_handles() {
    use super::{dpx_pdfnames::*, dpx_pdfobj::*};
    let _document = crate::compat::document::DocumentScope::new().unwrap();
    unsafe {
        let mut names = pdf_new_name_tree();
        let reference = pdf_names_reserve(names, b"later".as_ptr().cast(), 5);
        let number = pdf_new_number(17.0);
        assert_eq!(
            pdf_names_add_object(names, b"later".as_ptr().cast(), 5, number),
            0
        );
        let resolved = pdf_deref_obj(reference);
        assert_eq!(resolved, number);
        pdf_release_obj(resolved);
        pdf_release_obj(reference);
        pdf_delete_name_tree(&mut names);
    }
}

#[test]
fn resource_ids_and_references_survive_replacement_and_registry_growth() {
    use super::{dpx_pdfobj::*, dpx_pdfresource::*};
    let _document = crate::compat::document::DocumentScope::new().unwrap();
    crate::compat::document::with_diagnostics(|state| state.quiet = 2);
    unsafe {
        pdf_init_resources();
        let first = pdf_new_number(5.0);
        let id = pdf_defineresource(c"Font".as_ptr(), c"Shared".as_ptr(), first, 0);
        let old_reference = pdf_get_resource_reference(id);
        let replacement = pdf_new_number(6.0);
        assert_eq!(
            pdf_defineresource(c"Font".as_ptr(), c"Shared".as_ptr(), replacement, 0),
            id
        );
        assert_eq!(pdf_findresource(c"Font".as_ptr(), c"Shared".as_ptr()), id);
        assert_eq!(pdf_obj_typeof(first), PDF_OBJ_INVALID);
        assert!(pdf_deref_obj(old_reference).is_null());
        let new_reference = pdf_get_resource_reference(id);
        assert_ne!(pdf_compare_reference(old_reference, new_reference), 0);
        for index in 1..300 {
            let object = pdf_new_null();
            assert_eq!(
                pdf_defineresource(c"Font".as_ptr(), std::ptr::null(), object, 0),
                index
            );
        }
        let resolved = pdf_deref_obj(new_reference);
        assert_eq!(resolved, replacement);
        pdf_release_obj(resolved);
        pdf_close_resources();
        assert_eq!(pdf_obj_typeof(replacement), PDF_OBJ_INVALID);
        assert_eq!(pdf_findresource(c"Font".as_ptr(), c"Shared".as_ptr()), -1);
        pdf_release_obj(old_reference);
        pdf_release_obj(new_reference);
    }
}

#[test]
fn invalid_object_operations_return_errors_and_leave_handles_usable() {
    use super::dpx_pdfobj::*;
    let _document = crate::compat::document::DocumentScope::new().unwrap();
    unsafe {
        let number = pdf_new_number(7.0);
        assert_eq!(__pdf_entry_pdf_add_array(std::ptr::null_mut(), number), -1);
        assert_eq!(
            CStr::from_ptr(runtime::tt_pdf_last_error()),
            c"Missing PDF object"
        );
        assert_eq!(
            __pdf_entry_pdf_set_string(number, b"bad".as_ptr().cast_mut(), 3),
            -1
        );
        assert_eq!(
            CStr::from_ptr(runtime::tt_pdf_last_error()),
            c"Invalid PDF object type"
        );
        assert_eq!(pdf_number_value(number), 7.0);
        let array = pdf_new_array();
        pdf_add_array(array, number);
        assert_eq!(pdf_get_array(array, 0), number);
        pdf_release_obj(array);
    }
}

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
            use super::dpx_pdfobj::*;
            let cycle = pdf_new_array();
            let dictionary = pdf_new_dict();
            pdf_add_dict(
                dictionary,
                pdf_new_name(c"cycle".as_ptr()),
                pdf_link_obj(cycle),
            );
            pdf_add_array(cycle, dictionary);
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
fn aes_cbc_preserves_vectors_padding_and_block_boundaries() {
    use aes::cipher::{Array, BlockCipherDecrypt, KeyInit};
    unsafe {
        let key = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let iv: [u8; 16] = core::array::from_fn(|i| i as u8);
        let plain = [
            0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93,
            0x17, 0x2a,
        ];
        let mut encrypted = core::ptr::null_mut();
        let mut length = 0;
        AES_cbc_encrypt_tectonic(
            key.as_ptr(),
            key.len(),
            iv.as_ptr(),
            0,
            plain.as_ptr(),
            plain.len(),
            &mut encrypted,
            &mut length,
        );
        assert_eq!(
            hex(core::slice::from_raw_parts(encrypted, length)),
            "7649abac8119b246cee98e9b12e9197d"
        );
        libc::free(encrypted.cast());
        for key_len in [16, 24, 32] {
            let key = vec![0x35; key_len];
            for size in [0, 1, 15, 16, 17, 31, 32, 33] {
                let plain: Vec<_> = (0..size).map(|i| i as u8).collect();
                for padding in [0, 1] {
                    AES_cbc_encrypt_tectonic(
                        key.as_ptr(),
                        key_len,
                        iv.as_ptr(),
                        padding,
                        plain.as_ptr(),
                        size,
                        &mut encrypted,
                        &mut length,
                    );
                    let pad = if padding != 0 || size % 16 != 0 {
                        16 - size % 16
                    } else {
                        0
                    };
                    assert_eq!(length, size + pad);
                    let ciphertext = core::slice::from_raw_parts(encrypted, length);
                    let mut decoded = Vec::new();
                    let mut previous = iv;
                    for bytes in ciphertext.as_chunks::<16>().0 {
                        let mut block = Array::from(*bytes);
                        match key_len {
                            16 => aes::Aes128::new_from_slice(&key)
                                .unwrap()
                                .decrypt_block(&mut block),
                            24 => aes::Aes192::new_from_slice(&key)
                                .unwrap()
                                .decrypt_block(&mut block),
                            32 => aes::Aes256::new_from_slice(&key)
                                .unwrap()
                                .decrypt_block(&mut block),
                            _ => unreachable!(),
                        }
                        decoded.extend(block.iter().zip(previous).map(|(byte, old)| byte ^ old));
                        previous = *bytes;
                    }
                    assert_eq!(&decoded[..size], plain);
                    assert_eq!(&decoded[size..], vec![pad as u8; pad]);
                    libc::free(encrypted.cast());
                }
            }
        }
    }
}

#[test]
fn pdf_encryption_revisions_match_the_starting_revision() {
    let _document = crate::compat::document::DocumentScope::new().unwrap();
    unsafe {
        for (version, revision, key_size, expected) in [
            (1, 2, 5, "a907fc27477107574089d638c7600b214491098623ab"),
            (2, 3, 16, "d5476339c3ac87a9087028b0f52dbdfdbeb852a7892e"),
            (
                4,
                4,
                16,
                "f1f28e40d2d4ffbfb2110e803bb89d47e0cf915c3a725f196cdd97aa8a6ba0d4dcd025ec406ec958e77a6add075d8f27",
            ),
            (
                5,
                5,
                32,
                "f1f28e40d2d4ffbfb2110e803bb89d4749a2ef20ca5acb7ed07b6d19566c9c08cf38a6f510e820149b63d534d50f14d8",
            ),
            (
                5,
                6,
                32,
                "f1f28e40d2d4ffbfb2110e803bb89d4749a2ef20ca5acb7ed07b6d19566c9c08cf38a6f510e820149b63d534d50f14d8",
            ),
        ] {
            let mut security: super::dpx_pdfencrypt::pdf_sec = zeroed();
            security.V = version;
            security.R = revision;
            security.key_size = key_size;
            security.key = core::array::from_fn(|i| i as u8);
            security.label.objnum = 17;
            security.label.gennum = 2;
            super::dpx_mt19937ar::init_genrand(12345);
            let output =
                super::dpx_pdfencrypt::encrypt_owned(&mut security, b"PDF encryption fixture")
                    .unwrap();
            assert_eq!(hex(&output), expected);
        }
        let mut security: super::dpx_pdfencrypt::pdf_sec = zeroed();
        security.V = 5;
        security.key_size = 33;
        assert!(super::dpx_pdfencrypt::encrypt_owned(&mut security, b"invalid").is_err());
        security.key_size = 32;
        assert!(super::dpx_pdfencrypt::encrypt_owned(&mut security, b"valid").is_ok());
    }
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

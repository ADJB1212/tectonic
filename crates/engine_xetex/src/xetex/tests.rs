use super::teckit_c_engine::*;
use super::teckit_common::*;

#[test]
fn rust_normalization_preserves_state_across_input_and_output_chunks() {
    let input = "a\u{315}\u{300}\u{1100}\u{1161}\u{11a8}\u{9c7}\u{9be}\u{1d15e}\u{ffff}";
    for (form, expected) in [
        (
            kForm_NFC,
            "\u{e0}\u{315}\u{ac01}\u{9cb}\u{1d157}\u{1d165}\u{ffff}",
        ),
        (
            kForm_NFD,
            "a\u{300}\u{315}\u{1100}\u{1161}\u{11a8}\u{9c7}\u{9be}\u{1d157}\u{1d165}\u{ffff}",
        ),
    ] {
        for split in 0..=input.len() {
            unsafe {
                let mut converter = std::ptr::null_mut();
                assert_eq!(
                    TECkit_CreateConverter(
                        std::ptr::null_mut(),
                        0,
                        1,
                        kForm_UTF8,
                        kForm_UTF8 | form,
                        &mut converter,
                    ),
                    kStatus_NoError
                );
                let mut result = Vec::new();
                for (chunk, complete) in [
                    (&input.as_bytes()[..split], 0),
                    (&input.as_bytes()[split..], 1),
                ] {
                    let mut consumed = 0;
                    loop {
                        let mut output = [0; 4];
                        let mut input_used = 0;
                        let mut output_used = 0;
                        let status = TECkit_ConvertBuffer(
                            converter,
                            chunk[consumed..].as_ptr(),
                            (chunk.len() - consumed) as u32,
                            &mut input_used,
                            output.as_mut_ptr(),
                            output.len() as u32,
                            &mut output_used,
                            complete,
                        );
                        consumed += input_used as usize;
                        result.extend_from_slice(&output[..output_used as usize]);
                        if status == kStatus_OutputBufferFull {
                            assert!(input_used > 0 || output_used > 0);
                            continue;
                        }
                        assert_eq!(
                            status,
                            if complete == 0 {
                                kStatus_NeedMoreInput
                            } else {
                                kStatus_NoError
                            },
                            "split {split}, form {form}"
                        );
                        assert_eq!(consumed, chunk.len());
                        break;
                    }
                }
                assert_eq!(TECkit_DisposeConverter(converter), kStatus_NoError);
                assert_eq!(result, expected.as_bytes(), "split {split}, form {form}");
            }
        }
    }
}

#[test]
fn engine_entry_point_resolves_to_rust_startup() {
    assert_eq!(
        super::xetex_xetexd::tt_run_engine as *const (),
        super::xetex_runtime::tt_run_engine as *const (),
    );
    assert_eq!(
        super::xetex_xetexd::tt_cleanup as *const (),
        super::xetex_cleanup::tt_cleanup as *const (),
    );
}

#[test]
fn rust_and_native_rounding_exports_link_together() {
    unsafe extern "C" {
        fn tex_round(value: f64) -> i32;
    }

    for value in [-2147483649.0, -1.5, -0.5, 0.0, 0.49, 0.5, 1.5, 2147483648.0] {
        unsafe {
            assert_eq!(super::xetex_scaledmath::tex_round(value), tex_round(value));
        }
    }
}

#[test]
fn rust_converter_normalizes_unicode() {
    for (input, form, expected) in [
        ("a\u{308}", kForm_NFC, "\u{e4}"),
        ("\u{e4}", kForm_NFD, "a\u{308}"),
        ("\u{1100}\u{1161}\u{11a8}", kForm_NFC, "\u{ac01}"),
        ("\u{ac01}", kForm_NFD, "\u{1100}\u{1161}\u{11a8}"),
        ("a\u{315}\u{300}", kForm_NFC, "\u{e0}\u{315}"),
        ("\u{315}\u{300}", kForm_NFD, "\u{300}\u{315}"),
        ("\u{9c7}\u{9be}", kForm_NFC, "\u{9cb}"),
        ("\u{344}", kForm_NFC, "\u{308}\u{301}"),
        ("\u{1d15e}", kForm_NFD, "\u{1d157}\u{1d165}"),
        ("\u{ffff}", kForm_NFC, "\u{ffff}"),
    ] {
        unsafe {
            let mut converter = std::ptr::null_mut();
            assert_eq!(
                TECkit_CreateConverter(
                    std::ptr::null_mut(),
                    0,
                    1,
                    kForm_UTF8,
                    kForm_UTF8 | form,
                    &mut converter,
                ),
                kStatus_NoError
            );

            let mut output = [0; 32];
            let mut input_used = 0;
            let mut output_used = 0;
            let status = TECkit_ConvertBuffer(
                converter,
                input.as_ptr(),
                input.len() as u32,
                &mut input_used,
                output.as_mut_ptr(),
                output.len() as u32,
                &mut output_used,
                1,
            );
            assert_eq!(TECkit_DisposeConverter(converter), kStatus_NoError);
            assert_eq!(status, kStatus_NoError);
            assert_eq!(input_used as usize, input.len());
            assert_eq!(&output[..output_used as usize], expected.as_bytes());
        }
    }
}

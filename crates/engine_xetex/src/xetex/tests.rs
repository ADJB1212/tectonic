use super::teckit_c_engine::*;
use super::teckit_common::*;

#[test]
fn native_text_growth_handles_large_requests_in_one_allocation() {
    use super::xetex_xetex0_part4::native_text_capacity;
    assert_eq!(native_text_capacity(0, 1), 128);
    assert_eq!(native_text_capacity(128, 128), 256);
    assert_eq!(native_text_capacity(128, 100_000), 100_001);
    assert_eq!(
        native_text_capacity(i32::MAX / 2 + 1, i32::MAX - 1),
        i32::MAX
    );
    let mut capacity = 0;
    let mut growths = 0;
    for length in 1..100_000 {
        if capacity <= length {
            capacity = native_text_capacity(capacity, length);
            growths += 1;
        }
        assert!(capacity > length);
    }
    assert!(growths < 12);
}

#[test]
fn rust_hyphenation_minimum_clamps_boundaries() {
    for (value, expected) in [
        (i32::MIN, 1),
        (-1, 1),
        (0, 1),
        (1, 1),
        (2, 2),
        (62, 62),
        (63, 63),
        (64, 63),
        (255, 63),
        (i32::MAX, 63),
    ] {
        unsafe {
            assert_eq!(super::xetex_xetex0_part3::norm_min(value), expected);
        }
    }
}

#[test]
fn rust_format_byte_swapping_preserves_item_boundaries_and_round_trips() {
    for size in [1, 2, 4, 8, 16] {
        for count in [
            0,
            1,
            3,
            257,
            16 * 1024 * 1024 / size - 1,
            16 * 1024 * 1024 / size + 1,
        ] {
            for offset in [0, 1, 7] {
                let original: Vec<u8> = (0..count * size + offset + 1)
                    .map(|byte| byte as u8)
                    .collect();
                let mut expected = original.clone();
                if cfg!(target_endian = "little") {
                    for item in expected[offset..offset + count * size].chunks_exact_mut(size) {
                        item.reverse();
                    }
                }
                let mut actual = original.clone();
                unsafe {
                    let data = actual.as_mut_ptr().add(offset).cast();
                    super::xetex_ini_part2::swap_items(data, count, size);
                    assert_eq!(actual, expected, "item size {size}");
                    super::xetex_ini_part2::swap_items(data, 0, size);
                    assert_eq!(actual, expected, "empty item size {size}");
                    super::xetex_ini_part2::swap_items(data, count, size);
                }
                assert_eq!(actual, original, "round trip item size {size}");
            }
        }
    }
}

#[test]
#[ignore = "manual performance measurement"]
fn benchmark_format_byte_swapping() {
    use std::{hint::black_box, time::Instant};
    unsafe fn baseline<const N: usize>(mut ptr: *mut u8, count: usize) {
        for _ in 0..count {
            for index in 0..N / 2 {
                unsafe {
                    std::ptr::swap(ptr.add(index), ptr.add(N - 1 - index));
                }
            }
            ptr = unsafe { ptr.add(N) };
        }
    }
    for length in [64 * 1024, 4 * 1024 * 1024, 32 * 1024 * 1024] {
        let mut bytes = vec![42u8; length];
        for size in [2, 4, 8, 16] {
            for optimized in [false, true] {
                let start = Instant::now();
                for _ in 0..(256 * 1024 * 1024 / length) {
                    let ptr = black_box(bytes.as_mut_ptr());
                    unsafe {
                        if optimized {
                            super::xetex_ini_part2::swap_items(ptr.cast(), length / size, size);
                        } else {
                            match size {
                                2 => baseline::<2>(ptr, length / size),
                                4 => baseline::<4>(ptr, length / size),
                                8 => baseline::<8>(ptr, length / size),
                                16 => baseline::<16>(ptr, length / size),
                                _ => unreachable!(),
                            }
                        }
                    }
                    black_box(&bytes);
                }
                eprintln!(
                    "swap length={length} size={size} optimized={optimized}: {:?}",
                    start.elapsed()
                );
            }
        }
    }
}

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
fn teckit_endian_conversion_preserves_split_characters_and_small_output_buffers() {
    fn encode(text: &str, form: u16) -> Vec<u8> {
        match form {
            kForm_UTF8 => text.as_bytes().to_vec(),
            kForm_UTF16BE => text.encode_utf16().flat_map(u16::to_be_bytes).collect(),
            kForm_UTF16LE => text.encode_utf16().flat_map(u16::to_le_bytes).collect(),
            kForm_UTF32BE => text
                .chars()
                .flat_map(|c| (c as u32).to_be_bytes())
                .collect(),
            kForm_UTF32LE => text
                .chars()
                .flat_map(|c| (c as u32).to_le_bytes())
                .collect(),
            _ => unreachable!(),
        }
    }

    let text = "\0A\u{80}\u{7ff}\u{800}\u{ffff}\u{10000}\u{1f600}\u{10ffff}";
    let forms = [
        kForm_UTF8,
        kForm_UTF16BE,
        kForm_UTF16LE,
        kForm_UTF32BE,
        kForm_UTF32LE,
    ];
    for input_form in forms {
        let input = encode(text, input_form);
        for output_form in forms {
            let expected = encode(text, output_form);
            for split in 0..=input.len() {
                unsafe {
                    let mut converter = std::ptr::null_mut();
                    assert_eq!(
                        TECkit_CreateConverter(
                            std::ptr::null_mut(),
                            0,
                            1,
                            input_form,
                            output_form,
                            &mut converter
                        ),
                        kStatus_NoError
                    );
                    let mut actual = Vec::new();
                    for (chunk, complete) in [(&input[..split], 0), (&input[split..], 1)] {
                        let mut consumed = 0;
                        let mut capacity = 1;
                        let mut attempts = 0;
                        loop {
                            attempts += 1;
                            assert!(attempts < input.len() * 8 + 16);
                            let mut output = [0; 7];
                            let mut input_used = 0;
                            let mut output_used = 0;
                            let status = TECkit_ConvertBuffer(
                                converter,
                                chunk[consumed..].as_ptr(),
                                (chunk.len() - consumed) as u32,
                                &mut input_used,
                                output.as_mut_ptr(),
                                capacity,
                                &mut output_used,
                                complete,
                            );
                            consumed += input_used as usize;
                            actual.extend_from_slice(&output[..output_used as usize]);
                            if status == kStatus_OutputBufferFull {
                                capacity = capacity % 7 + 1;
                                continue;
                            }
                            assert_eq!(
                                status,
                                if complete == 0 {
                                    kStatus_NeedMoreInput
                                } else {
                                    kStatus_NoError
                                }
                            );
                            assert_eq!(consumed, chunk.len());
                            break;
                        }
                    }
                    assert_eq!(TECkit_DisposeConverter(converter), kStatus_NoError);
                    assert_eq!(
                        actual, expected,
                        "input {input_form}, output {output_form}, split {split}"
                    );
                }
            }
        }
    }
}

#[test]
fn engine_symbols_resolve_to_rust_implementations() {
    assert_eq!(
        super::xetex_xetexd::tt_run_engine as *const (),
        super::xetex_runtime::tt_run_engine as *const (),
    );
    assert_eq!(
        super::xetex_xetexd::tt_cleanup as *const (),
        super::xetex_cleanup::tt_cleanup as *const (),
    );
    assert_eq!(
        super::xetex_xetexd::main_control as *const (),
        super::xetex_xetex0_part4::main_control as *const (),
    );
    assert_eq!(
        super::xetex_xetexd::get_next as *const (),
        super::xetex_xetex0::get_next as *const (),
    );
    assert_eq!(
        super::xetex_xetexd::get_x_token as *const (),
        super::xetex_xetex0::get_x_token as *const (),
    );
    assert_eq!(
        (&raw const super::xetex_xetexd::eqtb).cast::<()>(),
        (&raw const super::xetex_ini::eqtb).cast::<()>(),
    );
}

#[test]
fn rust_rounding_export_handles_ties_and_limits() {
    unsafe extern "C" {
        fn tex_round(value: f64) -> i32;
    }

    for (value, expected) in [
        (-2147483649.0, i32::MIN),
        (-1.5, -2),
        (-0.5, -1),
        (0.0, 0),
        (0.49, 0),
        (0.5, 1),
        (1.5, 2),
        (2147483648.0, i32::MAX),
    ] {
        unsafe {
            assert_eq!(tex_round(value), expected);
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

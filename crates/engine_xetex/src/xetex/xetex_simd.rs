// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

#[inline]
pub(super) fn copy_ascii_line(input: &[u8], output: &mut [i32]) -> usize {
    let length = input.len().min(output.len());
    if length == 0 || !input[0].is_ascii() {
        return 0;
    }
    let mut offset = 0;
    // SAFETY: The minimum slice length bounds every unaligned load and store.
    // Check each vector before writing so the first marker and following output stay untouched.
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    unsafe {
        use std::arch::aarch64::*;
        while length - offset >= 16 {
            let bytes = vld1q_u8(input.as_ptr().add(offset));
            let newlines = vorrq_u8(vceqq_u8(bytes, vdupq_n_u8(b'\r')), vceqq_u8(bytes, vdupq_n_u8(b'\n')));
            if vmaxvq_u8(vorrq_u8(bytes, newlines)) >= 128 {
                break;
            }
            let low = vmovl_u8(vget_low_u8(bytes));
            let high = vmovl_high_u8(bytes);
            let dst = output.as_mut_ptr().add(offset).cast::<u32>();
            vst1q_u32(dst, vmovl_u16(vget_low_u16(low)));
            vst1q_u32(dst.add(4), vmovl_high_u16(low));
            vst1q_u32(dst.add(8), vmovl_u16(vget_low_u16(high)));
            vst1q_u32(dst.add(12), vmovl_high_u16(high));
            offset += 16;
        }
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use std::arch::x86_64::*;
        let zero = _mm_setzero_si128();
        while length - offset >= 16 {
            let bytes = _mm_loadu_si128(input.as_ptr().add(offset).cast());
            let newlines = _mm_or_si128(
                _mm_cmpeq_epi8(bytes, _mm_set1_epi8(b'\r' as i8)),
                _mm_cmpeq_epi8(bytes, _mm_set1_epi8(b'\n' as i8)),
            );
            if _mm_movemask_epi8(_mm_or_si128(bytes, newlines)) != 0 {
                break;
            }
            let low = _mm_unpacklo_epi8(bytes, zero);
            let high = _mm_unpackhi_epi8(bytes, zero);
            let dst = output.as_mut_ptr().add(offset);
            _mm_storeu_si128(dst.cast(), _mm_unpacklo_epi16(low, zero));
            _mm_storeu_si128(dst.add(4).cast(), _mm_unpackhi_epi16(low, zero));
            _mm_storeu_si128(dst.add(8).cast(), _mm_unpacklo_epi16(high, zero));
            _mm_storeu_si128(dst.add(12).cast(), _mm_unpackhi_epi16(high, zero));
            offset += 16;
        }
    }
    while offset < length {
        let byte = input[offset];
        if !byte.is_ascii() || byte == b'\r' || byte == b'\n' {
            break;
        }
        output[offset] = byte as i32;
        offset += 1;
    }
    offset
}

#[inline]
pub(super) fn eq_utf16_utf32(text: &[u16], scalars: &[i32]) -> bool {
    if text.len() != scalars.len() {
        return false;
    }
    let mut offset = 0;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    unsafe {
        use std::arch::aarch64::*;
        while text.len() - offset >= 8 {
            let units = vld1q_u16(text.as_ptr().add(offset));
            let low = vreinterpretq_u32_s32(vld1q_s32(scalars.as_ptr().add(offset)));
            let high = vreinterpretq_u32_s32(vld1q_s32(scalars.as_ptr().add(offset + 4)));
            let equal = vandq_u32(vceqq_u32(vmovl_u16(vget_low_u16(units)), low), vceqq_u32(vmovl_high_u16(units), high));
            if vminvq_u32(equal) != u32::MAX {
                return false;
            }
            offset += 8;
        }
        while text.len() - offset >= 4 {
            let units = vmovl_u16(vld1_u16(text.as_ptr().add(offset)));
            let values = vreinterpretq_u32_s32(vld1q_s32(scalars.as_ptr().add(offset)));
            if vminvq_u32(vceqq_u32(units, values)) != u32::MAX {
                return false;
            }
            offset += 4;
        }
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use std::arch::x86_64::*;
        let zero = _mm_setzero_si128();
        while text.len() - offset >= 8 {
            let units = _mm_loadu_si128(text.as_ptr().add(offset).cast());
            let low = _mm_loadu_si128(scalars.as_ptr().add(offset).cast());
            let high = _mm_loadu_si128(scalars.as_ptr().add(offset + 4).cast());
            let equal = _mm_and_si128(
                _mm_cmpeq_epi32(_mm_unpacklo_epi16(units, zero), low),
                _mm_cmpeq_epi32(_mm_unpackhi_epi16(units, zero), high),
            );
            if _mm_movemask_epi8(equal) != 0xffff {
                return false;
            }
            offset += 8;
        }
        while text.len() - offset >= 4 {
            let units = _mm_loadl_epi64(text.as_ptr().add(offset).cast());
            let units = _mm_unpacklo_epi16(units, _mm_setzero_si128());
            let values = _mm_loadu_si128(scalars.as_ptr().add(offset).cast());
            if _mm_movemask_epi8(_mm_cmpeq_epi32(units, values)) != 0xffff {
                return false;
            }
            offset += 4;
        }
    }
    text[offset..].iter().zip(&scalars[offset..]).all(|(&unit, &value)| unit as i32 == value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_line_copy_matches_scalar_for_every_byte_and_unaligned_buffers() {
        for length in [0, 1, 15, 16, 17, 31, 32, 33, 65] {
            for offset in 0..4 {
                for position in 0..length.max(1) {
                    for marker in 0..=255 {
                        let mut input: Vec<u8> = (0..offset + length).map(|index| 32 + (index * 17 % 96) as u8).collect();
                        if position < length {
                            input[offset + position] = marker;
                        }
                        for capacity in [length / 2, length, length + 1] {
                            let mut actual = vec![-1; offset + capacity + 1];
                            let mut expected = actual.clone();
                            let mut count = 0;
                            for &byte in input[offset..].iter().take(capacity) {
                                if !byte.is_ascii() || byte == b'\r' || byte == b'\n' {
                                    break;
                                }
                                expected[offset + count] = byte as i32;
                                count += 1;
                            }
                            assert_eq!(copy_ascii_line(&input[offset..], &mut actual[offset..offset + capacity]), count);
                            assert_eq!(
                                actual, expected,
                                "length={length} offset={offset} position={position} marker={marker} capacity={capacity}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    #[ignore = "manual performance measurement"]
    fn benchmark_mixed_width_comparison() {
        use std::{hint::black_box, time::Instant};

        fn baseline(text: &[u16], scalars: &[i32]) -> bool {
            if text.len() != scalars.len() {
                return false;
            }
            let mut offset = 0;
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            unsafe {
                use std::arch::aarch64::*;
                while text.len() - offset >= 4 {
                    let units = vmovl_u16(vld1_u16(text.as_ptr().add(offset)));
                    let values = vreinterpretq_u32_s32(vld1q_s32(scalars.as_ptr().add(offset)));
                    if vminvq_u32(vceqq_u32(units, values)) != u32::MAX {
                        return false;
                    }
                    offset += 4;
                }
            }
            #[cfg(target_arch = "x86_64")]
            unsafe {
                use std::arch::x86_64::*;
                while text.len() - offset >= 4 {
                    let units = _mm_loadl_epi64(text.as_ptr().add(offset).cast());
                    let units = _mm_unpacklo_epi16(units, _mm_setzero_si128());
                    let values = _mm_loadu_si128(scalars.as_ptr().add(offset).cast());
                    if _mm_movemask_epi8(_mm_cmpeq_epi32(units, values)) != 0xffff {
                        return false;
                    }
                    offset += 4;
                }
            }
            text[offset..].iter().zip(&scalars[offset..]).all(|(&unit, &value)| unit as i32 == value)
        }

        for length in [4, 8, 16, 32, 128, 1024] {
            let text: Vec<u16> = (0..length).map(|index| (index * 997) as u16).collect();
            for mismatch in [None, Some(0), Some(length - 1)] {
                let mut scalars: Vec<i32> = text.iter().map(|&unit| unit as i32).collect();
                if let Some(position) = mismatch {
                    scalars[position] = -1;
                }
                for (name, compare) in [("baseline", baseline as fn(&[u16], &[i32]) -> bool), ("SIMD", eq_utf16_utf32)] {
                    let start = Instant::now();
                    for _ in 0..(32 * 1024 * 1024 / length) {
                        black_box(compare(black_box(&text), black_box(&scalars)));
                    }
                    eprintln!("compare length={length} mismatch={mismatch:?} {name}: {:?}", start.elapsed());
                }
            }
        }
    }

    #[test]
    fn mixed_width_comparison_matches_scalar_for_offsets_tails_and_mismatches() {
        for length in 0..=65 {
            for offset in 0..8 {
                let text: Vec<u16> = (0..length + offset).map(|index| (index * 997) as u16).collect();
                let mut scalars: Vec<i32> = text.iter().map(|&unit| unit as i32).collect();
                let text = &text[offset..];
                assert!(eq_utf16_utf32(text, &scalars[offset..]));
                if length > 0 {
                    assert!(!eq_utf16_utf32(text, &scalars[offset..scalars.len() - 1]));
                }
                for position in 0..length {
                    let saved = scalars[offset + position];
                    for value in [-1, 0x10000, 0x10ffff, i32::MAX, saved ^ 1] {
                        scalars[offset + position] = value;
                        assert!(!eq_utf16_utf32(text, &scalars[offset..]));
                    }
                    scalars[offset + position] = saved;
                }
            }
        }
    }
}

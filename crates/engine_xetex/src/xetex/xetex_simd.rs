// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

#[inline]
pub(super) fn eq_utf16_utf32(text: &[u16], scalars: &[i32]) -> bool {
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
    text[offset..]
        .iter()
        .zip(&scalars[offset..])
        .all(|(&unit, &value)| unit as i32 == value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_width_comparison_matches_scalar_for_offsets_tails_and_mismatches() {
        for length in 0..=65 {
            for offset in 0..8 {
                let text: Vec<u16> = (0..length + offset)
                    .map(|index| (index * 997) as u16)
                    .collect();
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

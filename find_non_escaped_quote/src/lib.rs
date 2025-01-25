#![feature(portable_simd)]
#![allow(dead_code)]

use std::simd::{Simd, cmp::SimdPartialEq};

// expands to 01010101010...
const ODD_BITS: u64 = 0x5555555555555555;
const EVEN_BITS: u64 = !ODD_BITS;

#[inline]
fn find_odd_backslashes(input: Simd<u8, 64>, prev_run: &mut u64) -> u64 {
    // backslashes in this 64 byte chunk
    let bs_bits = input.simd_eq(Simd::splat(b'\\')).to_bitmask();
    // get the starting position of each run of bits
    let start_edges = bs_bits & !(bs_bits << 1);

    let even_start_mask = EVEN_BITS ^ *prev_run;
    let even_starts = start_edges & even_start_mask;
    let odd_starts = start_edges & !even_start_mask;
    let even_carries = bs_bits.wrapping_add(even_starts);

    let (mut odd_carries, ends_backslash) = bs_bits.overflowing_add(odd_starts);

    odd_carries |= *prev_run;

    *prev_run = u64::from(ends_backslash);
    let even_carry_ends = even_carries & !bs_bits;
    let odd_carry_ends = odd_carries & !bs_bits;
    let even_start_odd_end = even_carry_ends & ODD_BITS;
    let odd_start_even_end = odd_carry_ends & EVEN_BITS;
    even_start_odd_end | odd_start_even_end
}

#[cfg(test)]
mod tests {
    use std::simd::Simd;

    use utils::print_binary_number;

    use crate::find_odd_backslashes;

    #[test]
    fn da_test() {
        let epic_string = r#"{ "\\\""}\"#;
        let input = Simd::load_or_default(epic_string.as_bytes());

        let res = find_odd_backslashes(input, &mut 0);

        print_binary_number(res);
        assert!(res == 0);
    }
}

#![feature(portable_simd)]

use std::simd::{Simd, cmp::SimdPartialEq};

pub struct Input<'a> {
    input: &'a [u8],
}

impl<'a> Input<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input }
    }

    pub fn find_quotes_trailing_zeros(&self) -> Vec<usize> {
        let estimate = self.input.len() / 32;
        let mut res: Vec<usize> = Vec::with_capacity(estimate);
        let mut pos = 0;

        while pos < self.input.len() {
            let arr: Simd<u8, 64> = Simd::load_or_default(&self.input[pos..]);
            let mask = arr.simd_eq(Simd::splat(b'"')).to_bitmask();

            let mut current_mask = mask;
            while current_mask != 0 {
                let trailing = current_mask.trailing_zeros() as usize;
                res.push(pos + trailing);
                current_mask &= current_mask - 1;
            }

            pos += 64;
        }

        res
    }

    // Baseline non-SIMD implementation
    pub fn find_quotes_naive(&self) -> Vec<usize> {
        self.input
            .iter()
            .enumerate()
            .filter(|(_, c): &(usize, &u8)| **c == b'"')
            .map(|(i, _)| i)
            .collect()
    }
}

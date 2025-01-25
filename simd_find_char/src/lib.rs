#![feature(portable_simd)]
#![feature(allocator_api)]

use std::simd::{Simd, cmp::SimdPartialEq};

use utils::alloc::{bump_array::BumpArray, page_alloc::PageAllocator};

pub struct Input<'a> {
    input: &'a [u8],
}

impl<'a> Input<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input }
    }

    pub fn find_quotes_trailing_zeros(
        &self,
        alloc: &'a PageAllocator,
    ) -> BumpArray<usize, &PageAllocator> {
        let mut res = BumpArray::new(alloc, self.input.len());
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

    // Baseline non-SIMD implementation, slow af but kinda a vibe
    pub fn find_quotes_naive(&self) -> Vec<usize> {
        self.input
            .iter()
            .enumerate()
            .filter(|(_, c): &(usize, &u8)| **c == b'"')
            .map(|(i, _)| i)
            .collect()
    }
}

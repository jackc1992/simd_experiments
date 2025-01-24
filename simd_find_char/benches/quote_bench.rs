#![feature(allocator_api)]

use bump_array::BumpArray;
use criterion::{Criterion, criterion_group, criterion_main};
use page_alloc::PageAllocator;
use rand::{Rng, prelude::ThreadRng};
use simd_find_char::Input;
use utils::alloc::*;

fn create_test_data(size: usize, quote_frequency: f64, rng: &mut ThreadRng) -> Vec<u8> {
    let mut data = vec![b'x'; size];
    (0..size).for_each(|i| {
        if rng.gen_bool(1.0 / quote_frequency) {
            data[i] = b'"';
        }
    });
    data
}

fn criterion_benchmark(c: &mut Criterion) {
    // Test with different data sizes and quote frequencies
    let test_cases = vec![
        (1024, 64.0),
        (1024, 128.0),
        (1024 * 1024, 16.0),
        (1024 * 1024, 64.0),
        (1024 * 1024, 128.0),
    ];

    let mut rng: ThreadRng = rand::thread_rng();
    let alloc = PageAllocator::new(20 * GB);

    for (size, freq) in test_cases {
        let test_data = create_test_data(size, freq, &mut rng);
        let input = Input::new(&test_data);

        let mut group = c.benchmark_group(format!("quotes_size_{}_freq_{}", size, freq));

        group.bench_function("trailing_zeros", |b| {
            b.iter(|| input.find_quotes_trailing_zeros(&alloc))
        });

        group.bench_function("naive", |b| b.iter(|| input.find_quotes_naive()));

        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

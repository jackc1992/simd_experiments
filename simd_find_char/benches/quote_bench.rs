use criterion::{Criterion, criterion_group, criterion_main};
use simd_find_char::Input;

fn create_test_data(size: usize, quote_frequency: usize) -> Vec<u8> {
    let mut data = vec![b'x'; size];
    for i in (0..size).step_by(quote_frequency) {
        data[i] = b'"';
    }
    data
}

fn criterion_benchmark(c: &mut Criterion) {
    // Test with different data sizes and quote frequencies
    let test_cases = vec![
        (1024, 64),         // Small data, frequent quotes
        (1024, 128),        // Small data, sparse quotes
        (1024 * 1024, 64),  // Large data, frequent quotes
        (1024 * 1024, 128), // Large data, sparse quotes
    ];

    for (size, freq) in test_cases {
        let test_data = create_test_data(size, freq);
        let input = Input::new(&test_data);

        let mut group = c.benchmark_group(format!("quotes_size_{}_freq_{}", size, freq));

        group.bench_function("trailing_zeros", |b| {
            b.iter(|| input.find_quotes_trailing_zeros())
        });

        group.bench_function("naive", |b| b.iter(|| input.find_quotes_naive()));

        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

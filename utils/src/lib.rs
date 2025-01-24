#![feature(allocator_api)]

pub mod alloc;

pub fn print_binary_number(input: u64) {
    println!("{input:#066b}")
}

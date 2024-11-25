mod algorithms; // Refers to src/algorithms/mod.rs
mod rust_by_example; // Refers to src/rust_by_example/mod.rs

use crate::algorithms::quicksort::quicksort;

fn main() {
    // Integer sorting
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Original numbers: {:?}", numbers);
    quicksort(&mut numbers);
    println!("Sorted numbers: {:?}", numbers);
}
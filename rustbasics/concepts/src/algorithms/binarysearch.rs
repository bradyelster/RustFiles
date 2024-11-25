// An attribute to hide warnings for unused code.
#![allow(dead_code)]

pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = arr.len();

    while left < right {
        let mid: usize = left + (right - left) / 2;  // Avoid potential integer overflow
        
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None // target not found
}

fn main() {
    let arr: Vec<i32> = vec![1, 2, 5, 6, 10, 12, 17, 32, 55];
    let target: i32 = 55;
    
    match binary_search(&arr, target) {
        Some(index) => println!("Target {} found at index {} ✅", target, index),
        None => println!("Target {} not found in the given array ❌", target),
    }
}
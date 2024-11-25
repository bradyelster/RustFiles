fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    // Example usage with integers
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", numbers);
    
    bubble_sort(&mut numbers);
    
    println!("Sorted array: {:?}", numbers);

    // Example usage with strings
    let mut words = vec!["banana", "apple", "cherry", "date"];
    println!("Original array: {:?}", words);
    
    bubble_sort(&mut words);
    
    println!("Sorted array: {:?}", words);
}
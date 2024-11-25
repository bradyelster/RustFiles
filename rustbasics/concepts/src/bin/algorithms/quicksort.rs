fn quicksort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    let pivot_index = partition(arr);
    
    // Recursively sort left and right partitions
    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    
    let mut store_index = 0;
    for i in 0..len - 1 {
        if arr[i] < arr[len - 1] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    
    arr.swap(store_index, len - 1);
    store_index
}

fn main() {
    // Integer sorting
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Original numbers: {:?}", numbers);
    quicksort(&mut numbers);
    println!("Sorted numbers: {:?}", numbers);

    // String sorting
    let mut words = vec!["banana", "apple", "cherry", "date"];
    println!("Original words: {:?}", words);
    quicksort(&mut words);
    println!("Sorted words: {:?}", words);
}
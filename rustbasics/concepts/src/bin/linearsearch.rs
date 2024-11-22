fn linear_search(arr: &[i32], target: i32) {
    for (index, &value) in arr.iter().enumerate() { // Use `enumerate` to get index and value
        if value == target {
            println!("Target found at index {}.", index);
            return; // Exit the function as target is found
        }
    }
    println!("Target not found."); // Print if the target is not found
}

fn main() {
    let arr: Vec<i32> = vec![1, 2, 5, 6, 10, 12, 17, 32, 55];
    let target: i32 = 55;

    linear_search(&arr, target);
}
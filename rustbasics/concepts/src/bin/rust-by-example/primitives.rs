fn main() {
    let x: f64 = 1.0; // regular annotation
    let y = 1.0f64; // Suffix annotation

    println!("x is: {} and y is: {}", &x, &y);


    /* Compound types - Array and Tuple */
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple is a collection of values of different types 
    // and is constructed using parentheses ().
    let mut my_tuple: (u32, u8, bool, f32) = (5u32, 1u8, true, -5.04f32);
    // now let's mutate the first element in my_tuple (different value, same type)
    my_tuple.0 = 4u32; 

    println!("my_array is: {:#?}", &my_array); // {:#?} vertical print 
    println!("my_tuple is: {:?}", &my_tuple); // standard horizontal print
}
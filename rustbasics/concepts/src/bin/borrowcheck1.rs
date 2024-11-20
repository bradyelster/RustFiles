fn main() {
    let mut data: String = String::from("Hello, world!"); // Create a mutable string.

    // Immutable borrow.
    let first_borrow: &String = &data; // Borrow the string immutably.
    println!("First borrow: {}", first_borrow);

    // Mutable borrow.
    let second_borrow: &mut String = &mut data; // Borrow the string mutably.
    second_borrow.push_str(" Let's learn Rust!");

    // Uncommenting the next line will cause a borrow-checker error:
    // println!("After mutable borrow: {}", first_borrow);

    println!("Final data: {}", second_borrow);
}


/* Notes: & is a immutable reference. It allows you to read data but NOT modify it.
 *        &mut is a mutable reference. You can read and modify data.
 *        you can call the "&" sign the "reference operator" but it has no formal name. 
 *
 *        error is thrown if line 12 is uncommented since you cannot immutably borrow 
 *        if there is a mutable borrow active. let the mutable borrow go out of scope to "close out".
 *        see mut-borrow.rs for more on this.
 */
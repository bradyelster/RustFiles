fn main() {
    let mut data: String = String::from("Hello");

    // Mutable borrow
    {
        let mutable_ref: &mut String = &mut data; // Create a mutable reference
        mutable_ref.push_str(", world!"); // Use the mutable reference, append ", world!"
        // mutable_ref goes out of scope here
    }

    // Now we can immutably borrow `data`
    let immutable_ref: &String = &data;
    println!("Immutable reference: {}", immutable_ref);
}

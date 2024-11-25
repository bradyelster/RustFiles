fn main() {
    // define a variable `data` as the string "Hello"
    let mut data: String = String::from("Hi");

    {   
        // now define a new variable which borrows `data` with permission to mutate it
        let mutable_ref: &mut String = &mut data;

        // now use the new variable and append ", world!" to it
        // this changes the variable `data`
        mutable_ref.push_str(", Dad!");

        // mutable_ref goes out of scope here
    }

    // now we can immutably borrow `data`
    let immutable_ref: &String = &data;
    println!("Immutable reference: {}", immutable_ref);
}
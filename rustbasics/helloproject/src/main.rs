mod another_file;
mod folder1;
mod utils{
    pub mod core;
}

fn main() {
    // print something we made in main.rs
    println!("Hello, 🦀 from CARGO!");
    // now call function "func" from a file in src folder
    another_file::func();
    // now call function "anotherfunc" from folder1/mod.rs
    // note: file must be called "mod.rs" 
    folder1::anotherfunc(); 
    // now call function from nested file structure
    utils::core::yetanotherfunc();
}

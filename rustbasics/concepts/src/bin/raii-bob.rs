// Learning the basics of RAII (Resource Acquisition Is Initialization)
// from this Numberphile video: https://www.youtube.com/watch?v=pTMvh6VzDls
struct Bob {
    n: Vec<i32>,
}

impl Bob {
    fn new() -> Bob {
        Bob { n: Vec::new() }
    }
}

fn func_1() {
    let n: Bob = Bob::new();

    // "Read" the variable by performing a no-op action.
    println!("n is being read: {:?}", n.n); // Accessing `n` without modifying it.

    // Create new variable "m" which looks like it shares the same memory!
    let _m: Bob = n;
    // Behind the scenes, Rust performs a "move". "m" is now the owner
    // and "n" is not allowed to be used anymore! 
}

fn main() {
    func_1();
}

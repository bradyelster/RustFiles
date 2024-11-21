/* this part is called the Prelude */
extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;
/* this is the end of the Prelude */

fn main() {
    println!("Guess a number from 0 to 10.");

    let secret_number: i32 = rand::thread_rng().gen_range(1, 11);
    // println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess below:");

        let mut guess: String = String::new();
        //"the :: syntax means new() is an *associated function* of the String type"
        // associated functions are implemented on a type rather than a particular instance of a type
        // other languages call this a "static method"

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = guess.trim()
            .parse()
            .expect("Please input a number.");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Off by {}", (secret_number - guess).abs()),
            Ordering::Equal => {
                println!("You got it!, the number was: {}", secret_number);
                break;
            }
            Ordering::Greater => println!("Too big! Off by {}", (secret_number - guess).abs())
        }
    }
}
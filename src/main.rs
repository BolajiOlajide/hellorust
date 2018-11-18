extern crate rand;

use std::io; // library for obtaining user input from user
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world! - Welcome to earth!"); // macro used to print to the terminal
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop { // loop command is used to create an infinite loop
        println!("Please input your guess.");

        // variables in Rust are immutable by default
        // let foo = bar; basically declares foo as a constant (immutable)
        // let mut foo = bar; makes foo mutable
        let mut guess = String::new(); // String::new() returns a new instance of a string

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // The & indicates that this argument is a reference, which gives you a way to let multiple
        // parts of your code access one piece of data without needing to copy that data into memory
        // multiple times. References are a complex feature, and one of Rust’s major advantages is
        // how safe and easy it is to use references.

        // References like variables are also immutable by default hence why we write (&mut guess) instead
        // of (&guess).

        let guess: u32 = match guess.trim().parse() {
            // convert guess to an unsigned 32-bit integer
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        println!("")
    }
}

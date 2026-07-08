use std::cmp::Ordering;
use std::io;

use rand::RngExt;

fn main() {
    println!("Guess the number heheh!");

    let secret_number = rand::rng().random_range(1..=100);
    // rng returns a random number generator that is local to the current thread of execution and seeded by the OS

    loop {
         println!("Please input your guess.");

        let mut guess = String::new(); // all variables are immutable unless we declare otherwise with mut
        // above, the right-hand side of the expression returns a new instance of a String. :: indicates that new is an associated function of the String type.

        io::stdin() // stdin() returns an instance of std::io::stdin, a type representing a handle to the standard input for the terminal
            .read_line(&mut guess)
            .expect("Failed to process! Womp womp."); // expect is required for error handling here to avoid a compile warning

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Your guess has been recorded as {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

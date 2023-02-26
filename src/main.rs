use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Macro that prints a string to the screen.
    println!("Guess the number!");

    // Generate a random number in the (inclusive) range.
    // We use a random number generator which is local to the thread.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // In Rust, variables are immutable by default.
        // To make variable mutable, we add  mut before the variable name.
        // :: syntax indicates that new is an associated function of the String type.
        let mut guess = String::new();

        // stdin function returns an instance of std::io::Stdin (handle to the standard input).
        io::stdin()
            // Get input from the user and store it in guess variable (& indicates that argument is a reference).
            // Like variables, references are immutable hence we need to write '&mut guess'.
            .read_line(&mut guess)
            // In real app we should handle the error.
            .expect("Failed to read line.");

        // Convert String to an unsigned 32-bit number and shadow the previous guess variable.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Catch all errors.
            Err(_) => continue,
        };

        // We are using String with placeholder to print the guess value.
        println!("You guessed: {guess}");

        // Rust will infer that secret_number should be a u32 as well.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too BIG!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

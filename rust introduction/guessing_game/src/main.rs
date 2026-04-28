use std::io; // use the standard library's input/output module

use rand::Rng; // use the random number generation functionality from the rand crate

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // generate a random number between 1 and 100

    println!("Please input your guess.");

    let mut guess = String::new();  // mutable variable to store the user's guess

    io::stdin().read_line(&mut guess).expect("Failed to read line"); // read the user's input and store it in the guess variable, handling any potential errors

    println!("You guessed: {guess}"); //and print the user's guess to the console
}

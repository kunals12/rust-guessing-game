mod array;

// Import necessary modules from the standard library
use std::io; // For handling input/output operations
use std::cmp::Ordering; // For comparing numbers
use rand::Rng; // For generating random numbers

fn main() {
    // Print a welcome message to the player
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Start an infinite loop for the guessing game
    loop {
        println!("Please input your guess.");

        // Create a mutable String variable to store the user's guess
        let mut guess = String::new();

        // Read the user's input and store it in the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim the input and attempt to parse it as an unsigned 32-bit integer (u32)
        // If parsing fails, continue to the next iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Print the user's guess
        println!("You guessed: {guess}");

        // Compare the user's guess with the secret number
        match guess.cmp(&secret_number) {
            // If the guess is less than the secret number, print "Too small!"
            Ordering::Less => println!("Too small!"),
            // If the guess is greater than the secret number, print "Too big!"
            Ordering::Greater => println!("Too big!"),
            // If the guess is equal to the secret number, print "You win!" and break the loop
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    array::arrays_to_handle()
}

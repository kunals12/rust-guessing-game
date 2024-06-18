use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number:i8 = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // mutable variable that is currently bound to a new, empty instance of a String

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}

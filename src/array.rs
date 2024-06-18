use std::io; // Import the standard input/output library

pub(crate) fn arrays_to_handle() {
    let a = [1, 2, 3, 4, 5]; // Define an array of integers

    println!("Please enter an array index."); // Prompt the user to enter an array index

    let mut index = String::new(); // Create a mutable String variable to store the user's input

    // Read the user's input and store it in the index variable
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line"); // If reading the line fails, print an error message and panic

    // Trim the whitespace from the input, parse it to a usize, and handle any errors
    let index: usize = index
        .trim() // Remove any whitespace from the input
        .parse() // Attempt to parse the input as a usize (unsigned 32-bit integer)
        .expect("Index entered was not a number"); // If parsing fails, print an error message and panic

    // Access the element at the given index in the array
    let element = a[index];

    // Print the value of the element at the specified index
    println!("The value of the element at index {index} is: {element}");
}

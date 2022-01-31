use std::io;

fn main() {

    println!("########### Guess the number ###########");

    println!("Please input your guess.");

    // Let => Create a new variable
    // mut means that it is mutable (const is the default)
    let mut guess = String::new();

    // Read line from input, appending into the given String
    // The string is given as a mutable reference.
    // References are mutable, hence the &mut
    io::stdin()
    .read_line(&mut guess)
    .expect("Error during line read"); // Faillure Handling, will cause termination of program if hit

    // Print the guess with the {} placeholder syntax
    println!("Guess: {}", guess);
}

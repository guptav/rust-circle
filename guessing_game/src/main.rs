/// Guesssing Game

use std::io;


fn main() {
    println!("Guess the Number");
    println!("Enter the Number between 1 to 100 ?");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read the line");
    println!("You guessed: {}", guess);
}


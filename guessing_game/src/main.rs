/// Guesssing Game

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the Number (1 to 100)");
    let secret = rand::thread_rng().gen_range(1, 101);
    println!("Secret num is {}", secret);

    loop {
        println!("Enter your guess ?");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read the line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}


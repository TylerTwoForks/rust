use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Enter your guess: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("your random number is {secret_number}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    //this expect() line is actually called on the Result type of .read_line() which can bee OK or ERR. 
    //if it's ERR, this expect() will crash the program. if it's OK, that OK value just gets returned and is used in &mut guess. 
    //handling the error is best instead of crashing like this, but we'll get to that in later lessons. 

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("You guessed too low!"),
        Ordering::Equal => prinln!("Nailed it!"),
        Ordering::Greater => prinln!("oof, too high")
    }

}

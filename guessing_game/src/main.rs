use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("your random number is {secret_number}");

    //infinite loop for continued guessing until the correct guess is made. 
    println!("Enter Number: ");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //this expect() line is actually called on the Result type of .read_line() which can bee OK or ERR. 
        //if it's ERR, this expect() will crash the program. if it's OK, that OK value just gets returned and is used in &mut guess. 
        //handling the error is best instead of crashing like this, but we'll get to that in later lessons. 

        println!("You guessed: {guess}");

        //being able to re-use the same variable name here is called Shadowing. 
        // even though we had instantiated `guess` above, we can re-use it here and the compiler is ok with it. 
        // this functionality is often used when converting types.
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a Number!");
                continue;
            }

        };
        //calling .parse() here converts the guess variable to another type. 
        // whatever type is assigned to the variable, in our case `:i32`
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed too low!"),
            Ordering::Equal => {
                println!("Nailed it!");
                break;
            },
            Ordering::Greater => println!("oof, too high")
        }    
    }
    

}

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");
    
    println!("Please input your guess.");

    // Variable to store the user input in
    let mut guess= String::new();

    io::stdin()//create a handle on the input
        .read_line(&mut guess)//method used to capture user data
        .expect("Failed to read line");

    println!("You guessed: {guess}");

}
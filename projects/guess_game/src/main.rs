use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");
    
    loop{
        println!("Please input your guess.");

        // Variable to store the user input in
        let mut guess= String::new();

        io::stdin()//create a handle on the input
            .read_line(&mut guess)//method used to capture user data
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        // println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win !!");
                break;
            }
        }
    }


}
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please guess the number:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations, you guessed the correct number: {}", secret_number);
                break;
            }
        }
    }
}

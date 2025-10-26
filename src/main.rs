use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");
    println!("Can you guess it within 16 attempts?");

    const MAX_ATTEMPTS: u32 = 16;
    const MIN_NUMBER: u32 = 1;
    const MAX_NUMBER: u32 = 100;

    let secret_number: u32 = rand::thread_rng().gen_range(MIN_NUMBER..=MAX_NUMBER);
    let mut attempts: u32 = 0;

    // User must guess the secret number in 16 attempts.
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable empty string

        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input, please try again.");

        // Parse user input
        let input: u32 = match guess.trim().parse() {
            Ok(num) if (MIN_NUMBER..=MAX_NUMBER).contains(&num) => num,
            Ok(_) => {
                println!(
                    "Please type a number between {} and {}!",
                    MIN_NUMBER, MAX_NUMBER
                );
                continue;
            }
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        // Increment attempts
        attempts += 1; // count of number of guesses

        // compare user's input with secret_number
        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // Check for first guess
                if attempts == 1 {
                    println!(
                        "You win with Bonus! The secret number was {}. You guessed it in {} guess!",
                        secret_number, attempts
                    );
                } else {
                    println!(
                        "You win! The secret number was {}. You guessed it in {} guesses.",
                        secret_number, attempts
                    );
                }
                break;
            }
        }
        // User has 16 attempts to guess the secret number.
        if attempts == MAX_ATTEMPTS {
            println!("You lose! The secret number was {}", secret_number);
            break;
        } else if attempts == MAX_ATTEMPTS - 1 {
            // Check for last guess
            println!("You have only 1 guess left.");
        } else {
            println!("You have {} guesses left.", MAX_ATTEMPTS - attempts);
        }
    }

    // Exit the Game
    println!("Press Enter to exit the game...");
    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read Exit input");
}

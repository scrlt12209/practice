use std::io;
use rand::Rng;

fn main() {
    let secret_number: i32 = rand::rng().random_range(1..101);
    let mut i: i32 = 0;

    while i < 5 {
        println!("Guess {}", i + 1);
        println!("Guess the number between 1 and 100: ");
        let mut user_input: String = String::new();

        io::stdin().read_line(&mut user_input).expect("Some Input Error.");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input! Try again");
                continue;
            },
        };

        if user_input < secret_number {
            println!("Too small!");
        } else if user_input > secret_number {
            println!("Too big!");
        } else {
            break;
        }

        i += 1;
    }

    if i == 5 {
        println!("You have lost! The number was {}!", secret_number);
    } else {
        println!("You guessed it! It's {}!", secret_number);
    }
}

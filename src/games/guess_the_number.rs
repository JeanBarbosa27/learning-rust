use rand::Rng;
use std::cmp::Ordering;

use crate::cli;

pub fn run() {
    const RANGE_START: u32 = 1;
    const RANGE_END: u32 = 100;
    cli::output::show::title(
        "Welcome to \"Guess the number\" game!",
        &format!( "The rules are simple, we are going to randomly choose a number between {RANGE_START} and \
        {RANGE_END},\nso you need to guess it.")[..],
    );

    let mut tries_count = 0;
    let secret_number: u32 = rand::thread_rng().gen_range(RANGE_START..=RANGE_END);

    loop {
        let guess: String = cli::input::from_user::generic(
            &format!("Please input your guess number between {RANGE_START} and {RANGE_END}:")[..],
        );

        tries_count += 1;

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => {
                if number > RANGE_END {
                    println!("This number exceeds the the edge.");
                    continue;
                }

                number
            }
            Err(_) => {
                println!("Please type a valid number between {RANGE_START} and {RANGE_END}");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The secret is higher than it."),
            Ordering::Greater => println!("The secret number is lower than it."),
            Ordering::Equal => {
                println!("Congratulations you won! It was {tries_count} tries.");
                break;
            }
        }
    }
}

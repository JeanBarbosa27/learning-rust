use crate::cli;

use super::fixed_variation_options;

pub fn run() {
    struct Person {
        name: String,
        birth_year: Option<u16>,
    }

    impl Person {
        fn age(&self) -> Option<u16> {
            // TODO: Improve it to use date time with Chrono
            match self.birth_year {
                Some(year) => Some(2024 - year),
                None => {
                    println!("It isn't a valid birth year");
                    None
                }
            }
        }
    }

    cli::output::show::title("Welcome to the sample for key value object usage!", "");
    let name: String = cli::input::from_user::generic("What's your name?");
    let birth_year: Option<u16> = Some(
        cli::input::from_user::generic("What's your birth year?")
            .parse()
            .expect("Couldn't parse to a number"),
    );

    let user: Person = Person { name, birth_year };
    // TODO: Get different roles according to age ranges
    println!(
        "Hi {}, since you are {:?} years old, here are the available roles:",
        user.name,
        user.age().expect("")
    );
    fixed_variation_options::run();
}

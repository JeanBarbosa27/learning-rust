mod cli;
mod games;
mod working_with;

use cli::input;

fn main() {
    cli::clear();
    cli::output::show::title("Welcome to Learning Rust", "");
    cli::output::show::menu(
        "Please select one of the options below:",
        vec![
            String::from("Play Guess The Number game"),
            String::from("An example of struct usage"),
            String::from("An example of HashMap usage"),
        ],
    );

    let choice: &str =
        &input::from_user::generic("\nWhich one do you want? (tip: press any other key to exit)");

    // TODO: may be interesting in the future refactoring it with a Trait
    match choice {
        "1" => {
            cli::clear();
            games::guess_the_number::run();
        }
        "2" => {
            cli::clear();
            working_with::struct_sample::run();
        }
        "3" => {
            cli::clear();
            working_with::hash_map_sample::run();
        }
        _ => println!("Exiting the program!"),
    }
}

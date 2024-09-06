use std::io;

pub fn generic(question: &str) -> String {
    println!("{}", question);
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Read line failed!");
    String::from(user_input.trim())
}

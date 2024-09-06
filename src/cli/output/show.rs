pub fn title(title: &str, subtitle: &str) {
    println!("\n\n{:+^80} \n", format!(" {} ", title));

    if !subtitle.is_empty() {
        // TODO: It should be interesting having a function to take a string and a certain amount
        // of characters to enforce line break;
        println!("{subtitle}\n");
        println!("{}", String::from("+").repeat(80));
    }
}
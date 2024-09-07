pub mod input;
pub mod output;
pub mod paginator;

pub fn clear() {
    let mut clear_command: &str = "clear";
    if cfg!(windows) {
        clear_command = "cls";
    }

    std::process::Command::new(clear_command).status().unwrap();
}

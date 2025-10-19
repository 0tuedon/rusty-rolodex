use crate::cli::Commands;

pub fn is_email_valid(email: &str) -> bool {
    let at_index = email.find("@");
    let dot_index = email.find(".");

    if dot_index > at_index {
        return true;
    }
    false
}

pub fn parse_input(input: &str) -> Option<Commands> {
    match input.trim().to_lowercase().as_str() {
        "add" => Some(Commands::Add),
        "list" => Some(Commands::List),
        "delete" => Some(Commands::Delete),
        "exit" => Some(Commands::Exit),
        _ => None,
    }
}

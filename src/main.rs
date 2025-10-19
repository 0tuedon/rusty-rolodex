use rusty_rolodex::cli::command_line::execute_command;
use rusty_rolodex::cli::Commands;
use rusty_rolodex::store::Contact;
use rusty_rolodex::validation::parse_input;
use std::io;

fn main() {
    let mut contacts_vec: Vec<Contact> = Vec::new();

    loop {
        println!("Please enter a command");
        let mut user_input = String::from("").to_lowercase();
        io::stdin()
            .read_line(&mut user_input)
            .expect("This is not a string");

        match parse_input(&user_input) {
            Some(Commands::Exit) => {
                println!("Exiting the CLI");
                break;
            }
            Some(Commands::List) => {
                println!("{:#?}", contacts_vec)
            }
            Some(cmd) => execute_command(cmd, &mut contacts_vec),
            None => {
                println!("No Command of such, ending the program");
                break;
            }
        }
    }
}

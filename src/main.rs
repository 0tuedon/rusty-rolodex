use std::io;

#[derive(Debug)]
struct Contact {
    name: String,
    phone: u128,
    email: String,
}

enum Commands {
    Add,
    List,
    Delete,
    Exit,
}

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

fn is_email_valid(email: &String) -> bool {
    let at_index = email.find("@");
    let dot_index = email.find(".");

    if dot_index > at_index {
        return true;
    }

    false
}

// this should be able to convert the string to an enum type of command
fn parse_input(input: &str) -> Option<Commands> {
    match input.trim().to_lowercase().as_str() {
        "add" => Some(Commands::Add),
        "list" => Some(Commands::List),
        "delete" => Some(Commands::Delete),
        "exit" => Some(Commands::Exit),
        _ => None,
    }
}

fn execute_command(cmd: Commands, contacts_vec: &mut Vec<Contact>) {
    match cmd {
        Commands::Add => {
            let mut current_contact = Contact {
                name: String::from(""),
                phone: 0,
                email: String::from("Tuedon"),
            };
            let contact_fields = [
                String::from("name"),
                String::from("phone"),
                String::from("email"),
            ];

            for field in &contact_fields {
                if field.as_str() == contact_fields[0] {
                    loop {
                        let mut user_contact = String::from("").to_lowercase();
                        println!("please enter the {} of the contact", field);
                        io::stdin()
                            .read_line(&mut user_contact)
                            .expect("Not a String");

                        if user_contact.len() > 1 {
                            let trimmed_user = user_contact.trim().to_string();
                            if trimmed_user.chars().all(char::is_alphabetic) {
                                current_contact.name = trimmed_user;
                                break;
                            }
                        }
                    }
                }
                if field.as_str() == contact_fields[1] {
                    loop {
                        let mut user_contact = String::from("").to_lowercase();
                        println!("please enter the {} of the contact", field);
                        io::stdin()
                            .read_line(&mut user_contact)
                            .expect("Not a String");

                        let number_trimmed = user_contact.trim();
                        if number_trimmed.len() > 9 {
                            let phone: Result<u128, _> = number_trimmed.parse();

                            match phone {
                                Ok(number) => {
                                    current_contact.phone = number;
                                    break;
                                }
                                Err(e) => {
                                    panic!("Failed to parse number {}", e)
                                }
                            }
                        }
                    }
                }
                if field.as_str() == contact_fields[2] {
                    loop {
                        let mut user_contact = String::from("").to_lowercase();
                        println!("please enter the {} of the contact", field);
                        io::stdin()
                            .read_line(&mut user_contact)
                            .expect("Not a String");

                        if is_email_valid(&user_contact) {
                            current_contact.email = user_contact.trim().to_string();
                            break;
                        }
                    }
                }
            }
                contacts_vec.push(current_contact);
        }

        Commands::Delete => {
            let mut user_contact = String::from("").to_lowercase();
            println!("please enter the name of the contact to delete");
            io::stdin()
                .read_line(&mut user_contact)
                .expect("Not a String");
            contacts_vec.retain(|contact| contact.name == user_contact);
        }
        Commands::Exit => {
            println!("Exit");
        }
        Commands::List => {
            println!("List");
        }
    }
}

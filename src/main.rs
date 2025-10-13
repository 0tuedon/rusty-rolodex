use std::io;

#[derive(Debug)]
struct Contact {
    name: String,
    phone: u128,
    email: String,
}

struct Commands (String,String);

fn main() {
    let commands = Commands(String::from("add"), String::from("list"));
    let  mut contacts_vec: Vec<Contact> =  Vec::new();
    let contact_fields  = [String::from("name"), String::from("phone"), String::from("email")];

    loop {
        println!("Please enter a command");
        let mut user_input = String::from("");
        io::stdin().read_line(&mut user_input).expect("This is not a string");

        if user_input.trim() == commands.0  {
            let mut current_contact = Contact{name:String::from(""), phone:0, email:String::from("Tuedon")};

            for field in &contact_fields  {
                 let mut user_contact = String::from("");
                println!("please enter the {} of the contact", field);
                io::stdin().read_line(&mut user_contact).expect("Not a String");

                if field.as_str() == contact_fields[0]{
                    current_contact.name = user_contact.trim().to_string();
                }
                if field.as_str() == contact_fields[1]{
                    let number_trimmed =  user_contact.trim();
                    let  phone:Result<u128,_> = number_trimmed.parse();
                    println!("{:?}", number_trimmed);
                    match phone {
                        Ok(number) => {current_contact.phone = number}
                        Err(e) => {panic!("Failed to parse number {}",e)}
                    }
                }
                if field.as_str() == contact_fields[2]{
                    current_contact.email = user_contact.trim().to_string();
                }
            }

            contacts_vec.push(current_contact);

        }
        else if user_input.trim() == commands.1 {
            println!("{:#?}", contacts_vec)
        }
        else {
            break;
        }
    }
}

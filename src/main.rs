use std::io;

#[derive(Debug)]
struct Contact {
    name: String,
    phone: u128,
    email: String,
}

struct Commands (String,String, String);

fn main() {
    let commands = Commands(String::from("add"), String::from("list"), String::from("exit"));
    let  mut contacts_vec: Vec<Contact> =  Vec::new();
    let contact_fields  = [String::from("name"), String::from("phone"), String::from("email")];

    loop {
        println!("Please enter a command");
        let mut user_input = String::from("").to_lowercase();
        io::stdin().read_line(&mut user_input).expect("This is not a string");

        if user_input.trim() == commands.0  {
            let mut current_contact = Contact{name:String::from(""), phone:0, email:String::from("Tuedon")};

            for field in &contact_fields  {


                if field.as_str() == contact_fields[0]{
                    loop{
                        let mut user_contact = String::from("").to_lowercase();
                        println!("please enter the {} of the contact", field);
                        io::stdin().read_line(&mut user_contact).expect("Not a String");

                        if user_contact.len() > 1 {
                            let trimmed_user = user_contact.trim().to_string();
                            if trimmed_user.chars().all(char::is_alphabetic) {
                                  current_contact.name = trimmed_user;
                                  break;
                            }
                         }



                    }


                }
                if field.as_str() == contact_fields[1]{


                    loop {
                        let mut user_contact = String::from("").to_lowercase();
                        println!("please enter the {} of the contact", field);
                        io::stdin().read_line(&mut user_contact).expect("Not a String");

                        let number_trimmed =  user_contact.trim();
                        if number_trimmed.len() > 9 {
                            let  phone:Result<u128,_> = number_trimmed.parse();

                            match phone {
                                Ok(number) => {
                                    current_contact.phone = number;
                                    break;
                                }
                                Err(e) => {panic!("Failed to parse number {}",e)}
                            }
                        }

                    }

                }
                if field.as_str() == contact_fields[2]{


                   loop {
                       let mut user_contact = String::from("").to_lowercase();
                       println!("please enter the {} of the contact", field);
                       io::stdin().read_line(&mut user_contact).expect("Not a String");

                       if is_email_valid(&user_contact){
                            current_contact.email = user_contact.trim().to_string();
                            break;
                       }

                   }

                }
            }

            contacts_vec.push(current_contact);

        }
        else if user_input.trim() == commands.1 {
            println!("{:#?}", contacts_vec)
        }
        else if user_input.trim() == commands.2 {
            break;
        }
        else {
            println!("Inavlid input, try again");
        }
    }
}


fn is_email_valid(email: &String) -> bool {
    let at_index = email.find("@");
   let  dot_index = email.find(".");

       if dot_index >  at_index {
           return true;
       }

    false
}

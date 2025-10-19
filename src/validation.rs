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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert!(is_email_valid("test@example.com"));
    }

    #[test]
    fn test_invalid_email() {
        assert!(!is_email_valid("not-an-email"));
    }

    #[test]
      fn test_parse_input_add() {
          assert_eq!(parse_input("add"), Some(Commands::Add));
      }

      #[test]
      fn test_parse_input_list() {
          assert_eq!(parse_input("LIST"), Some(Commands::List));
      }

      #[test]
      fn test_parse_input_delete_with_whitespace() {
          assert_eq!(parse_input("  delete  "), Some(Commands::Delete));
      }

      #[test]
      fn test_parse_input_exit() {
          assert_eq!(parse_input("exit"), Some(Commands::Exit));
      }

      #[test]
      fn test_parse_input_invalid() {
          assert_eq!(parse_input("unknown"), None);
      }
}

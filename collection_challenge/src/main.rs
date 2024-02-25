use std::io::stdin;

fn main() {
    println!("Welcome to the CLI employee interface!");

    println!("For instructions, enter help. Otherwise enter a command");

    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).expect("Error");

        let sanitized_input = input.trim();
        
        match sanitized_input {
            "help" => { display_help_message() }
            "exit" => {  
                println!("Exiting program");
                break; 
            }
            _ => {
                if handle_command(sanitized_input) {
                    println!("This might be an acceptable phrase")
                } else {
                    println!("Command not recognized. Type help for more information or exit to close the program")
                }
            }
        };
    }
}

fn display_help_message() {
    println!("Available commands:");
    println!("1) You can add a user by using the following statement:");
    println!("  Add 'x' to 'y'");
    println!("  ** Where x is a user, and y is the department");
    println!("2) You can see what people are in a department currently by using the following statement:");
    println!("  View users from 'x'");
    println!("  ** Where x is the department");
}

fn handle_command(user_input: &str) -> bool {
        let first_idx: usize = 0; 
        for (idx, word) in user_input.split_whitespace().enumerate() {
            if idx == first_idx {
                match word {
                    "Add" => {
                        handle_add();
                        return true;
                    }
                    "View" => {
                        handle_view();
                        return true;
                    },
                    _ => { return false }
                }
            }
        }
        false
    }

fn handle_add() {
    println!("Add command!")
}

fn handle_view() {
    println!("View command!")
}


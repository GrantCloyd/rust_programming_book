use std::io::stdin;
use std::collections::HashMap;


fn main() {
    println!("Welcome to the CLI employee interface!");
    println!("For instructions, enter help. Otherwise enter a command");
    let mut department_users: HashMap<String, Vec<String>> = HashMap::new();
    
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
            _ => { commands::handle_command(sanitized_input, &mut department_users) }
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

pub mod commands;
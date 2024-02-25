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
            _ => { handle_command(sanitized_input) }
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

fn handle_command(user_input: &str) {
    let first_word = user_input.split_once(" ").unwrap_or_default().0;

    match first_word {
            "Add" => {
                handle_add(&user_input);
            }
            "View" => {
                handle_view(&user_input);
            },
            _ => { println!("Command not recognized. Type help for more information or exit to close the program") }
        }
}

fn handle_add(input: &str) {
    let iter = input.split_whitespace();

    if check_for_length(iter.clone().count()) {
        let user = iter.clone().nth(1).unwrap();
        let department = iter.last().unwrap();
        println!("Add command! {user} added to {department}")
    } else {
        println!("Add command not formatted properly")
    }
}

fn handle_view(input: &str) {
    let iter = input.split_whitespace();
    
    if check_for_length(iter.clone().count()) {
        let department = iter.last().unwrap();
        println!("View command! Viewing users from {department}")
    } else {
        println!("View command not formatted properly")
    }
}

fn check_for_length(count: usize) -> bool {
    count == 4 as usize
}

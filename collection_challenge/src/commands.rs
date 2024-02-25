use std::{collections::HashMap, str::SplitWhitespace};

pub fn handle_command(user_input: &str, department_users: &mut HashMap<String, Vec<String>>) {
    let first_word = user_input.split_once(" ").unwrap_or_default().0;

    match first_word {
        "Add" => {
            add::handle_add(&user_input, department_users);
        }
        "View" => {
            view::handle_view(&user_input, department_users);
        },
        _ => { println!("Command not recognized. Type help for more information or exit to close the program") }
    }
}

fn format_error(error_type: &str){
    println!("{error_type} command not formatted properly")
}

fn check_for_length(count: usize) -> bool {
    count == 4 as usize
}

fn get_last_word(iter:  SplitWhitespace<'_>) -> String {
  let result = iter.last().unwrap().to_string();
  result
}

mod add;
mod view;

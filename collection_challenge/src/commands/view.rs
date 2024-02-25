use std::{collections::HashMap, str::SplitWhitespace};
    
pub fn handle_view(input: &str, department_users: &mut HashMap<String, Vec<String>>) {
    let iter = input.split_whitespace();
    if super::check_for_length(iter.clone().count()) {
       view_users(iter, department_users)
    } else {
        super::format_error("View")
    }
}

fn view_users(iter: SplitWhitespace<'_>, department_users: &mut HashMap<String, Vec<String>>){
    let department = iter.last().unwrap().to_string();
    let users = department_users.get(&department);

    match users {
        Some(vect) => {
            println!("Users from {}:", department);
            for user in vect {
                println!("{user}")
            }
        },
        None => {
            println!("This department does not exist, please add it first.")
        }
    }
}
use std::{collections::HashMap, str::SplitWhitespace};
    
pub fn handle_add(input: &str, department_users: &mut HashMap<String, Vec<String>>) {
    let mut iter = input.split_whitespace();

    if super::check_for_length(iter.clone().count()) {
        add_user(iter, department_users);
    } else {
        super::format_error("Add")
    }
}

fn add_user(iter: SplitWhitespace<'_>, department_users: &mut HashMap<String, Vec<String>>){
    let user = iter.clone().nth(1).unwrap().to_string();
    let department = super::get_last_word(iter);

    println!("Adding {user} to {department}");
    let current_entry = department_users.entry(department).or_insert(vec!());
    current_entry.push(user)
}
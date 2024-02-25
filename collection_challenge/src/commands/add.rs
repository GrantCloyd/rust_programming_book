use std::{collections::HashMap, str::SplitWhitespace};
    
pub fn handle_add(input: &str, department_users: &mut HashMap<String, Vec<String>>) {
    let mut iter = input.split_whitespace();

    if super::check_for_length(iter.clone().count()) {
        add_user(&mut iter, department_users);
    } else {
        super::format_error("Add")
    }
}

fn add_user(iter: &mut SplitWhitespace<'_>, department_users: &mut HashMap<String, Vec<String>>){
    let user = iter.nth(1).unwrap().to_string();
    let department = iter.last().unwrap().to_string();

    println!("Adding {user} to {department}");
    let current_entry = department_users.entry(department).or_insert(vec!());
    current_entry.push(user)
}
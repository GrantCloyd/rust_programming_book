fn main() {
    let s = String::from("hello");
    takes_ownership(&s);
    println!("{}", s);

    let x: i32 = 5;
    makes_copy(x);
    println!("{}", x);

    let len = calculate_length(&s);
    println!("The lenth of '{}' is {}.", s, len);

    let mut mut_str = String::from("bout to change");
    change(&mut mut_str);
    println!("{}", mut_str)
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn calculate_length(s: &String) -> usize {
  // let length =  s.len();
   //length
   s.len()
}

fn change(s: &mut String) {
    s.push_str(" to this");
}


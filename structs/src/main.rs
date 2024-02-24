struct User {
    username: String, 
    email: String,
    sign_in_count: u64,
    active: bool,
}

// can create tuple structs that will help with typing 
// with similar looking objects - ex below, a function
// looking for Point tuple can't be passed a Color tuple
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);


fn main() {
    let mut user1 = build_user("yo@gmail.com".to_string(), "Yo".to_string());

    let name = user1.username;
    println!("{}", name);
    user1.username = String::from("Yo1000");
    println!("{}", name);
    println!("{}", user1.username);

    let user3 = User {
        email: String::from("me@gmail.com"),
        username: String::from("meeee"),
        ..user1
    };

    println!("{}", user3.active)
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
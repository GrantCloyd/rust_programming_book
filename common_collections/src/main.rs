use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    let  a = [1,2,3];

    //let mut v: Vec<i32> = Vec::new();

    // v.push(1);
    // v.push(2);
    // v.push(3);

    {
        let v2 = vec![1,2,3];
    }

    let v =  vec![1,2,3,4,5];

    let third = &v[2];

    println!("The third element is: {}", third);

    // get method allows for a safer way to handle out of bounds as a vec is stored on heap
    // vec, unlike array is stored on heap so it is unknown what it's size
    // is at run time
    match v.get(10) {
        Some(third) => println!("The third element is {}", third),
        None => println!("Element is missing"),
    }
    
    let mut v2 = vec![1,2,3,4,5];

    for i in &mut v2 {
        *i += 50;
        println!("{}", i)
    }

    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 is actually borrowed here and could not be referenced after
    let s3 : String = s1 + &s2;

    let hello: String = String::from("hello");

    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }

    let russian_hello = "Здравствуйте";

    for g in russian_hello.graphemes(true) {
        println!("{g}")
    }

//    hash_map_main();
    hello_world_string_mapping_main();

}

fn hash_map_main() {
    let blue: String = String::from("Blue");
    let yellow: String = String::from("Yellow");

    let mut scores = HashMap::new();

    // note, this also backwards defines the scores's type
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);    

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 20);

    scores.entry(String::from("Yellow")).or_insert(30);
}


fn hello_world_string_mapping_main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}

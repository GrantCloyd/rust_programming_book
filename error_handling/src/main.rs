
//use std::env;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
  //  env::set_var("RUST_BACKTRACE", "1");
    //panic!("crash and burn!");
    //a();

    //match syntax for error handling
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file, 
    //     Err(err) => {
    //         match err.kind() {
    //             ErrorKind::NotFound => match File::create("hello.txt") {
    //                 Ok(fc) => fc, 
    //                 Err(e) => panic!("Problem creating the file: {:?}", e)
    //             }
    //             other_error => {
    //                 panic!("Problem creating the file: {:?}", other_error)
    //             }
    //         }
    //     }
    // };

    // will panic if fails, if not will return the file
    // unwrap() works similar - just without message
   // let f = File::open("hello.txt").expect("Will blow up if hello.txt not present");


   

}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    // can add ? to end of f for similar result
    // let mut f = match f {
    //     Ok(file) => file, 
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();

    // match f.read_to_string(&mut s){
    //     Ok(_) => Ok(s), 
    //     Err(e) => Err(e)
    // }

    f.read_to_string( &mut s)?;
    Ok(s)

    // method chaining version
    //let mut s = String::new();
    // File::open('hello.txt')?.read_to_string(&mut s)?;
    //Ok(s)
    
    // convenience function that opens and read strings together with fewer imports
    // fs::read_to_string("hello.txt")

}

// fn a(){
//     b();
// }

// fn b() {
//     c(22)
// }

// fn c(num: i32) {
//     if num == 22 {
//         panic!("Don't pass in 22!")
//     }
// }
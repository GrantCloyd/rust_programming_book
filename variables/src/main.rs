fn main() {
   let x = 5;

   println!("{x}");

    let x = 6;

    println!("is now {x}");
    
    const SUBSCRIBER_COUNT : u32 = 100_000;

    // note debug error

    let tup = ("Let's Get Rusty", 100_000);

    let (channel, sub_count) = tup;
    let dot_notation_str = tup.0;

    println!("Channel: {channel}, Subscriber Count: {sub_count}");
    println!("Channel: {dot_notation_str}, Subscriber Count: {}", tup.1.to_string());

    let error_codes = [200, 404, 500 ];
    //let not_found = error_codes[1];

    let val = my_function(12, 24);
    println!("Value returned as {val}");

    let res = counter();
    println!("{res}");

    lift_off();
    iterate();
}

                                // need to define return type with arrow notation
fn my_function(x: i32, y: i32) -> i32 {
    let z = x+y; 
    println!("{x} + {y} = {z}");
    z
}

fn counter() -> String {
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        if counter > 10 {break};
    }
    return "Result".to_string()
}

fn lift_off() {
    let mut number: i32 = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFT OFF")
}

fn iterate() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}")
    }

}

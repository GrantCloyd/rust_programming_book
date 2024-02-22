// mod is defined in this file, but module fns, etc live in front_of_house.rs ie same name as module
mod front_of_house;

fn serve_order() {

}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String, 
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup, 
        Salad
    }

    fn fix_incorrect_order(){
        cook_order();
        super::serve_order()
    }

    fn cook_order() {

    }
}

use std::env::set_current_dir;

pub use crate::front_of_house::hosting;
// could also do use crate::front_of_house::hosting::add_to_waitlist to just bring fn into scope

// bring in multiple items with nested path
use rand::{Rng, CryptoRng, ErrorKind::Transient };


use std::io::{self, Write};
// glob operator brings all into scope
// use std::io::*;

pub fn eat_at_restuarant() {
    // absolute
    crate::front_of_house::hosting::add_to_waitlist();
    
    //relative
    front_of_house::hosting::add_to_waitlist();

    // note need to make toast public 
    let mut meal = back_of_house::Breakfast::summer("rye bread");

     meal.toast = String::from("wheat");


    // app enum must be made pub, but only need to make pub the enum, not every type
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1, 10);
}


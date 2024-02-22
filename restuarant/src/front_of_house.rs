// similar idea as in lib.rs, hosting is defined here, but rust looks for a child file, hosting.rs file 
// in a folder with the name, front_of_house - kind of railsesque in terms of name conventions

pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}

}

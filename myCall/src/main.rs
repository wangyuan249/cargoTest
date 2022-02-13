extern crate wangzy97lib;

use wangzy97lib::utils::*;

fn main() {
    println!("Rust Lib Function Call Test... ");
    it_works();
    add_class(1);
    add_student("Allen");
    add_student("Steven");
    add_student("Mark");

    take_attendance();
}


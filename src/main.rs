use crate::lib::int_input;
use crate::lib::str_input;

mod lib;

fn main() {
    println!("Hello, world!");
    let user_name = str_input("What is your name: ");
    let user_age = int_input("How old are you: ");
    println!("User's name is: {}", user_name);
    println!("User's age is: {}", user_age);
    let sum = user_age + user_name.len() as i32;
    println!("{}", sum)
}
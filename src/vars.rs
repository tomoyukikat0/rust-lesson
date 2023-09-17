use std::char::MAX;

pub mod sub_a;
mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars modue!!");
    let mut x = 5;
    println!("The value of x is: {}", x);

    println!("Memory address of x is: {:p}", &MAX_POINTS);
}

// Rust program to count numbers

use std::io;

fn main() {
    println!("\nEnter lower bound");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read value");
    let lower_bound: i32 = input1.trim().parse().expect("Input is not a number");

    println!("\nEnter upper bound");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read value");
    let upper_bound: i32 = input2.trim().parse().expect("Input is not a number");
    
    for x in lower_bound..upper_bound { // Upper bound not included
        println!("Count level is {}", x);
    }
}

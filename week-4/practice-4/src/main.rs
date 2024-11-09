// Rust program to determine age pass

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Failed to enter a string");
    
    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Failed to enter a string");
    let age:u32 = input2.trim().parse().expect("Failed to enter a number");

    if age >= 18 {
        println!("Welcome to the party {}", input1);
    } else {
        println!("Oops, you are not eligible for this party {}", input1);
    }

}

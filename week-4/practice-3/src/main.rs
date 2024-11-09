// Rust program to calculate the area of a triangle with a given base and height

use std::io; // Access Rust Standard Library

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nEnter the base of the triangle:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let base:f32 = input1.trim().parse().expect("Not a valid number");

    println!("\nEnter the height of the triangle:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let height:f32 = input2.trim().parse().expect("Not a valid number");

    if base > 0.0{
        let area:f32 = (base * he1ight) / 2.0;
        println!("The area of the triangle is: {}", area);
    }
}

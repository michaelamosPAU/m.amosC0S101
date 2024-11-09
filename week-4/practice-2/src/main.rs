// Rust program to calculate the area of a triangle given the three sides

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    println!("Hello, world!");

    println!("Enter the first side of the triangle:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Not a valid integer");

    println!("Enter the second side of the triangle:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Not a valid integer");

    println!("Enter the third side of the triangle:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Not a valid integer");

    let s:f32 = (a + b + c) / 2.0;
    let mut area:f32 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt();

    println!("The area of the triangle is: {}", area);
}

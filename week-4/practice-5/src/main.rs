// Rust program to read the height of a person and then print if the person is tall, dwarf, or average
use std::io;

fn main() {
    let mut input = String::new();

    println!("\nPlease enter your height (in centimeters):");
    io::stdin().read_line(&mut input).expect("Failed to enter string");
    let height:f32 = input.trim().parse().expect("Failed to  enter number");

    if height >= 150.0 && height <= 170.0{
        println!("You are of average height");
    }
    else if height > 170.0 && height <= 195.0{
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0{
        println!("You are a dwarf");
    }
    else {
        println!("You have abnormal height");
    }

}

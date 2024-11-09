// Rust program to output name and age
use std::io; 

fn main() {
    println!("\nStudent Information Management System!");

    input name
    println!("\nPlease enter your name");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name) // referencing the mutable string we declared earlier
        .expect("Failed to read the input");
    
    println!("Your name is {}", name);

    // input age
    println!("\nEnter your age");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Input is not an integer"); // Convert the string to an integer and ti check if the value is not an integer with the error message
    println!("Your age is: {}", age);

}


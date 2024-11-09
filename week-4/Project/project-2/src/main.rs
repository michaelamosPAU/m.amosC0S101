// Rust program to determine annual incentive from experience and age

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nEnter your level of experience (Input 0 for inexperienced and 1 for inexperience)");
    io::stdin().read_line(&mut input1).expect("Failed to read value");
    let experience: i32 = input1.trim().parse().expect("Input is not a number");

    println!("\nEnter you age");
    io::stdin().read_line(&mut input2).expect("Failed to read value");
    let age: u32 = input2.trim().parse().expect("Input is not a number");

    if experience == 1 && age >= 40{
        println!("Your annual incentive is N1,560,000");
    }
    else if experience == 1 && age >= 30 && age < 40{
        println!("Your annual incentive is N1,480,000");
    }
    else if experience == 1 && age < 28 {
        println!("Your annual incentive is N1,300,000");
    }
    else if experience == 0 {
        println!("Your incentive is N100,000");
    }
}

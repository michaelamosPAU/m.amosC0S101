use std::io;

fn add(a: i32, b: i32) {
    let sum = a + b;

    println!("Sum of A and B = {}", sum);
}

fn main() {
    let mut input = String::new();
    println!("Enter input parameter for A:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: i32 = input.trim().parse().expect("Not a valid integer");

    let mut input2 = String::new();
    println!("Enter input parameter for B:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: i32 = input2.trim().parse().expect("Not a valid integer");

    add(a, b);
}

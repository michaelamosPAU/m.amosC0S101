// Rust program to determine the roots of a quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value for a");
    io::stdin().read_line(&mut input1).expect("Failed to read value");
    let a:f32 = input1.trim().parse().expect("Failed to input");

    println!("Enter the value for b");
    io::stdin().read_line(&mut input2).expect("Failed to read value");
    let b:f32 = input2.trim().parse().expect("Failed to input");

    println!("Enter the value for c");
    io::stdin().read_line(&mut input3).expect("Failed to read value");
    let c:f32 = input3.trim().parse().expect("Failed to input");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);

        println!("The roots are {} and {}", root1, root2);
    }
    else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The root is {}", root);
    }
    else {
        println!("The equation has no real roots.");
    }
}

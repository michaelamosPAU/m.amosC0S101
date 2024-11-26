// Mathematical Calculator
use std::io;

fn trapezium_formula(){
    println!("\nArea of a trapezium");
    println!("Please enter the height of the Trapezium");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f32 = height.trim().parse().expect("Failed to convert input to float");
    
    println!("Please enter the length of the first base of the Trapezium");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let base1: f32 = base1.trim().parse().expect("Failed to convert input to float");

    println!("Please enter the length of the second base of the Trapezium");
    let mut base2 = String::new();
    io::stdin().read_line(&mut base2).expect("Failed to read input");
    let base2: f32 = base2.trim().parse().expect("Failed to convert input to float");

    let area: f32 = (height / 2.0) * (base1 + base2);
    println!("\nThe area of the Trapezium is {}", area);
}

fn rhombus_formula(){
    println!("Area of a Rhombus");
    println!("Please enter the length of the diagonal1 of the Rhombus");
    let mut diagonal1 = String::new();
    io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
    let diagonal1: f32 = diagonal1.trim().parse().expect("Failed to convert input to float");

    println!("Please enter the length of the diagonal2 of the Rhombus");
    let mut diagonal2 = String::new();
    io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
    let diagonal2: f32 = diagonal2.trim().parse().expect("Failed to convert input to float");

    let area: f32 = 0.5 * diagonal1 * diagonal2;
    println!("\nThe area of the rhombus is: {}", area);
}

fn parallelogram(){
    println!("Area of a Parallelogram");
    println!("Please enter the base of the Parallelogram");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let base: f32 = base.trim().parse().expect("Failed to convert input to float");

    println!("Please enter the height of the Parallelogram");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let altitude: f32 = altitude.trim().parse().expect("Failed to convert input to float");

    let area: f32 = base * altitude;
    println!("\nThe area of the parallelogram is: {}", area);
}

fn cube_formula(){
    println!("\nArea of a Cube");
    println!("Please enter the length of the side of the Cube");
    let mut side = String::new();
    io::stdin().read_line(&mut side).expect("Failed to read input");
    let side: f32 = side.trim().parse().expect("Failed to convert input to float");

    let area: f32 = 6.0 * side.powf(2.0);
    println!("\nThe area of the cube is: {}", area);
}

fn cylinder_formula(){
    println!("Volume of a Cylinder");
    println!("Please enter the radius of the cylinder");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Failed to read input");
    let radius: f32 = radius.trim().parse().expect("Failed to convert input to float");

    println!("Please enter the height of the cylinder");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f32 = height.trim().parse().expect("Failed to convert input to float");

    let volume: f32 = std::f32::consts::PI * radius.powf(2.0) * height;
    println!("\nThe volume of the cylinder is {}", volume)
}

fn main() {
    println!("Welcome to the Calculator Program");
    println!("\n1 = Area of Trapezium formula");
    println!("2 = Area of Rhombus");
    println!("3 = Area of Parallelogram");
    println!("4 = Area of Cube");
    println!("5 = Volume of Cylinder");

    let mut condition = String::new();

    while condition.trim() != "1" {
        println!("\nEnter your choice (Select one choice from the numbers above 1, 2, 3, 4, 5)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: i32 = choice.trim().parse().expect("Input is not an integer");
    
        if choice < 1 || choice > 5 {
            println!("The number you selected is not part of the available option. Please try again");
            continue
        }
    
        match choice {
            1 => trapezium_formula(),
            2 => rhombus_formula(),
            3 => parallelogram(),
            4 => cube_formula(),
            5 => cylinder_formula(),
            _ => println!("Unable to solve the equation")
        }
    
        println!("\nWould like to perform another equation. Type 0 for yes and 1 for no");
        io::stdin().read_line(&mut condition).expect("Failed to read value");
    }
}


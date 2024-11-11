//Rust program to determine the eligibilty of voting for 50 candidates

use std::io; // Importing Rust standard library

fn main(){
    // Loop statement for 50 candidates    
    for _x in 1..50{
        // Serial Number for candidate
        println!("\nNumber {}", _x); // Print index number of staff
        
        // Input values 
        let mut name = String::new();
        let mut email = String::new();
        let mut department = String::new();
        let mut state_of_origin = String::new();
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        
        // Read information from user response
        println!("\nPlease enter your name");
        io::stdin().read_line(&mut name).expect("Failed to read input");

        println!("\nPlease enter your email address");
        io::stdin().read_line(&mut email).expect("Failed to read input");
    
        println!("\nPlease enter you department");
        io::stdin().read_line(&mut department).expect("Failed to read input");
    
        println!("\nPlease enter your state of origin");
        io::stdin().read_line(&mut state_of_origin).expect("Failed to read input");
    
        println!("\nAre you a class rep. (If yes select 1, if not select 0)");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let class_rep:i32 = input1.trim().parse().expect("Failed to input");
    
        println!("\nAre you in 100 level. (If yes select 1, if not select 0)");
        io::stdin().read_line(&mut input2).expect("Failed to read");
        let level_100:i32 = input2.trim().parse().expect("Failed to input");
    
        println!("\nIs your CGPA above 4.0 (If yes select 1, if not select 0)");
        io::stdin().read_line(&mut input3).expect("Failed to read");
        let cgpa:i32 = input3.trim().parse().expect("Failed to input");

        // Conditional statements for the results
        if class_rep == 1 && level_100 == 0 && cgpa == 1 {
            println!("\n{}{}{}{}You can vote", name, email, department, state_of_origin);
        } else {
            println!("\nSorry, you are not eligible to vote");
        }
    }
}
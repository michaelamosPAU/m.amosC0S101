// Rust program to determine the amount of incentive of 100 staff members

use std::io; // Import Rust standard library

fn main(){
    // Loop statement for only 100 staff members
    for _x in 1..100{
        println!("\nNumber {}", _x); // Print index number of staff
        
        // Input values
        let mut name = String::new();
        let mut number_of_papers = String::new();
        
        // Read information from user response
        println!("\nEnter your name: ");
        io::stdin().read_line(&mut name).expect("Failed to read input");
    
        println!("\nEnter the number of papers have published: ");
        io::stdin().read_line(&mut number_of_papers).expect("Failed to read input");
        let number_of_papers: i32 = number_of_papers.trim().parse().expect("Input is not an integer");
    
        // Conditional Statement to calcuate the incentive for staff
        if 3 <= number_of_papers && number_of_papers <= 5 {
            println!("\nHello {}The incentive you received is ₦500,000", name);
        }
        else if 5 < number_of_papers && number_of_papers < 10 {
            println!("\nHello {}The incentive you received is ₦800,000", name);
        }
        else if number_of_papers > 10 {
            println!("\nHello {}The incentive you received is ₦1,000,000", name);
        }
        else if number_of_papers < 3{
            println!("\nHello {}The incentive you received is ₦100,000", name);
    
        }
    }

}
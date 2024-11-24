// Question 1
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
    
        println!("\nWhat level are you in? (100, 200, 300, 400, 500)");
        io::stdin().read_line(&mut input2).expect("Failed to read");
        let level_100:i32 = input2.trim().parse().expect("Failed to input");
    
        println!("\nWhat is your CGPA? (0.0 - 5.0))");
        io::stdin().read_line(&mut input3).expect("Failed to read");
        let cgpa:f64 = input3.trim().parse().expect("Failed to input");

        // Conditional statements for the results
        if class_rep == 1 && level_100 != 100 && cgpa > 4.0 {
            println!("\n{}{}{}{}You can vote", name, email, department, state_of_origin);
        } else {
            println!("\nSorry, you are not eligible to vote");
        }
    }
}

// Question 2
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
        else if number_of_papers >= 10 {
            println!("\nHello {}The incentive you received is ₦1,000,000", name);
        }
        else if number_of_papers < 3{
            println!("\nHello {}The incentive you received is ₦100,000", name);
    
        }
    }

}
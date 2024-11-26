use std::io;

fn main() {
    println!("\nWelcome to our Restaurant!\n\nTake a look at our menu filled with tasty delicacies you would enjoy with their prices\n\nP = Poundo Yam/Edinkaiko Soup - ₦3,200\nF = Fried rice and chicken - ₦3,000\nA = Amala & Ewedu Soup - ₦2,500\nE = Eba & Egusi Soup - ₦2,000\nW = White rice & stew - ₦2,500");

    let mut totalamount: f32 = 0.0;
    let mut condition = String::new();

    while condition.trim() != "1" {
        let mut input = String::new();
        println!("\nWhat would you like to eat (Select one out the letter P, F, A, E, W as shown above for the item you want)?");
        io::stdin().read_line(&mut input).expect("Failed to read value");
        let choice = input.trim().to_lowercase(); // Convert to lowercase directly

        if !["p", "f", "a", "e", "w"].contains(&choice.as_str()) {
            println!("Invalid Input. Please choose P, F, A, E, or W.");
            continue;
        }

        let mut quantity = String::new();
        println!("\nHow many portions of {} would you like to order?", match choice.as_str() {
            "p" => "Poundo Yam/Edinkaiko Soup",
            "f" => "Fried rice and chicken",
            "a" => "Amala & Ewedu Soup",
            "e" => "Eba & Egusi Soup",
            "w" => "White rice & stew",
            _ => "Invalid choice", // This will never be reached due to earlier check
        });

        io::stdin().read_line(&mut quantity).expect("Failed to read value");
        let quantity_num: u32 = quantity.trim().parse().expect("Please enter a valid number");

        match choice.as_str() {
            "p" => totalamount += 3200.00 * quantity_num as f32,
            "f" => totalamount += 3000.00 * quantity_num as f32,
            "a" => totalamount += 2500.00 * quantity_num as f32,
            "e" => totalamount += 2000.00 * quantity_num as f32,
            "w" => totalamount += 2500.00 * quantity_num as f32,
            _ => println!("Invalid choice! Please select P, F, A, E, or W."),
        }

        println!("\nWould that be all you want to order? (Select 1 for Yes and 0 for No)");
        io::stdin().read_line(&mut condition).expect("Failed to read value");
    }

    // Apply discount if the total amount is greater than 10,000
    if totalamount > 10000.0 {
        println!("\nThank you for coming\nYour total amount is: ₦{:.2}", totalamount * 0.95);
    } else {
        println!("\nThank you for coming\nYour total amount is: ₦{:.2}", totalamount);
    }
}

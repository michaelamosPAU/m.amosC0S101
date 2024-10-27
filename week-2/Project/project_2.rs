// Question 2

// Created a default struct for each record
struct Product{
    item: String,
    quantity: f32,
    amount: f32
}

fn main() {
    // Define a vector for product structs
    let mut products: Vec<Product> = Vec::new();
    
    // Add instances of Product to the vector
    products.push(Product {
        item: String::from("Toshiba"),
        quantity: 2.0,
        amount: 450000.00,
    });
    
    products.push(Product {
        item: String::from("Mac"),
        quantity: 1.0,
        amount: 1500000.00,
    });
    
    products.push(Product {
        item: String::from("HP"),
        quantity: 3.0,
        amount: 750000.00,
    });
    
    products.push(Product {
        item: String::from("Dell"),
        quantity: 3.0,
        amount: 2850000.00,
    });
    
    products.push(Product {
        item: String::from("Acer"),
        quantity: 1.0,
        amount: 250000.00,
    });
    
    // Define the Total quantity and total amount
    let mut total_quantity = 0.0;
    let mut total_amount = 0.0;
    
    // Loop through structs
    for product in &products{
        total_quantity += product.quantity;
        total_amount += product.amount;
    };
    
    // Calculate Average
    let total_average = total_amount / total_quantity;
    
    println!("The total sum of money from the sales record is ₦{}", total_amount);
    println!("The average sum of money from the sales record is ₦{}", total_average);
 }
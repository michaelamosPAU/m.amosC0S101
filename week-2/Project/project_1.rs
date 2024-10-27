// Question 1
fn main() {
    let p: f32 = 520000000.0; // Principal Amount
    let r: f32 = 0.1; // Rate in decimal
    let n: u32 = 5; // Number of years
    
    // Defining the base and raising it to the power of `n`
    let base: f32 = 1.0 + r; // Defined the base
    let a = p * (base.powf(n as f32));
    
    let cl = a - p;
    println!("The compound interest after 5 years is ₦{}", cl); // prints hello
}
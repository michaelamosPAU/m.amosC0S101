fn main() {
    let p: f32 = 210000.00; // Principal Amount
    let r: f32 = 0.05; // Rate in decimal
    let n: u32 = 3; // Number of years
    
    // Defining the base and raising it to the power of `n`
    let base: f32 = 1.0 - r; // Defined the base
    let a = p * (base.powf(n as f32));
    
    // Prints the value of the TV
    println!("The value of the TV after 3 years is ₦{}", a);
}   
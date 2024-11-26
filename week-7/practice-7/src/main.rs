fn main() {
    // Array with data type {explicitinteger datatype}
    let arr1:[i32; 4] = [10,20,30,40];
    println!("\nArray with datatype");
    println!("array is {:?}", arr1);
    println!("The length of the array is {}", arr1.len());

    // Array without datatype (implicit float data type)
    let arr2 = [19.4, 30.7, 30.4, 40.9, 51.2, 72.2];
    println!("\nArray without datatype");
    println!("array is {:?}", arr2);
    println!("Array size is {}", arr2.len());

    // Array with default values that creates  and initializes all its elements with a default value of 1.
    let arr3:[i32;8] = [-1;8];
    println!("\nArray with default values");
    println!("array is {:?}", arr3);
    println!("Array size is {}", arr3.len());
}

fn main() {
    let num:i32 = 5;
    mutate_num_to_zero(num);
    println!("The value of the no is: {}", num)
}

fn mutate_num_to_zero(mut num: i32) {
    num = num * 0;
    println!("Param num value is : {}", num);
}

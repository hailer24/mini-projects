use std::io;

fn factorial(num:u64) -> u64 {
    if num <= 1 {
        return 1;
    }
    num * factorial(num-1)
}

fn main() {
    println!("Enter a Number");
    let mut input_text:String = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read input");

    let num:u64 = input_text.trim().parse::<u64>().expect("Please Enter a number");

    println!("The factorial is {}", factorial(num));
}
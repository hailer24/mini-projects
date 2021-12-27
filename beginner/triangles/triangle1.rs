use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a Number");
    io::stdin().read_line(&mut input).expect("Error taking input.");
    let num = input.trim().parse::<i64>().expect("Invalid number.");

    for i in 0..num+1 {
        for _ in 0..i{
            print!("*");
        }
        print!(" \n ");
    }
}
//Create a program which return input as a sum of two prime numbers
use std::io;


fn is_prime(num:i64) -> bool {
    if num == 2 {return true;}
    if num%2 == 0 {
        return false;
    }

    for i in 3..num/2 {
        if num%i == 0 {
            return false;
        }
    }

    true
}

fn get_input() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("failed to read line");
    inp
}

fn main() {
    let num = get_input().trim().parse::<i64>().unwrap();

    for i in 0..num/2 + 1 {
        if is_prime(i) && is_prime(num-i){
            println!("{} + {}", i, num-i);
        }
    }
    

}

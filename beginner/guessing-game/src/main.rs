use rand::Rng;
use rand::thread_rng;
use std::io;

fn guess(&num:&i32) -> bool {

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Error reading");

    let n = input_text.trim().parse::<i32>().expect("Error parsing");

    if num == n {
        println!("Correct");
        return true;
    }

    if num < n {
        println!("The number is higher");
    }
    if num > n {
        println!("The number is lower");
    }

    false
}


fn main() {
    let num = thread_rng().gen_range(0,100);

    loop {
        println!("Guess the random number");
        if guess(&num) {
            break;
        }

    }
}

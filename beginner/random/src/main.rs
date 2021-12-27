use rand::Rng;
use rand::thread_rng;

fn main() {
    let x = thread_rng().gen_range(1,3);
    match x {
        1 => println!("fn 1"),
        2 => println!("fn 2"),
        3 => println!("fn 3"),
        _ => println!("fn _"),
    }
}
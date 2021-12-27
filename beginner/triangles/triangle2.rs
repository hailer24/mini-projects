use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number");
    io::stdin().read_line(&mut input).expect("Error reading");
    let num = input.trim().parse::<u32>().expect("Error parsing");
    
    for i in 0..num+1 {
        let mut str = String::new();
        for j in 0..num+1 {
            if j > i {str.push('*');}
            
        } 
        println!("{}", str);
    }
}
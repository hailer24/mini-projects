use std::io;

fn get_input() -> u64 {
    print!("Enter a number ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");

    let ret:u64 = input.trim().parse::<u64>().expect("Error parsing input");
    ret
    
} 




fn main() {
    let num:u64 = get_input();
    
    let mut vect:Vec<u64> = vec![1,1];

    while vect.last().unwrap() <= &num {
        vect.push(vect[vect.len() - 1] + vect[vect.len() - 2]);
    }
    
    if vect.last().unwrap() > &num {vect.pop();}
    for x in vect.iter() {
        print!("{} ", x);
    }
    
    
}

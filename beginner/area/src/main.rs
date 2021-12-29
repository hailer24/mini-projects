use std::io;

fn input_nums() -> i64 {
    let mut input_text = String::new();
    io::stdin()
    .read_line(&mut input_text)
    .expect("Error parsing input");
    let ret:i64 = input_text
    .trim()
    .parse::<i64>()
    .expect("Input is not a number");
    ret
}

fn main() {
    println!("1.Diameter\n2.radius");
    let num = input_nums();
    let radius = (input_nums() as f64)/(num as f64);

    println!("Area {:20.2}\nperimeter {:15.2}", std::f64::consts::PI* radius*radius, std::f64::consts::PI*radius*2f64);
    
}

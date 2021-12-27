use std::io;

const CONV:f64 =  75.04;

fn convert(amount:f64,choice: &str) ->f64 {
    match choice {
        "dollar" => amount * CONV,
        "Rupees" => amount / CONV,
        _ => amount,
    }
}

fn main() {
    println!("Choose from the options:");
    println!("1. Rupees to Dollars");
    println!("2. Dollars to Rupees");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Error reading input");

    let num = input_text.trim().parse::<u64>().expect("Invalid number");
    
    let choice;
  let sign;
  let converted_sign;

  match num {
    1 => {
      choice = "Rupees";
      sign = "₹";
      converted_sign = "$";
    },

    2 => {
      choice = "dollars";
      sign = "$";
      converted_sign = "₹";
    },

    _ => {
      println!("Please choose an option from the list");
      return;
    }
  };
  println!("Enter amount to convert:");
  let mut amount_text = String::new();
  io::stdin()
    .read_line(&mut amount_text)
    .expect("Failed to read input");

  let amount = amount_text.trim().parse::<f64>().expect("That's not a number");

  println!("{}{} is {}{:.2}", sign, amount, converted_sign, convert(amount, choice));

    }
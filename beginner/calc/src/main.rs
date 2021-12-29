mod calc;


fn main() {
    println!("Enter a sum, or 'q' to quit:");

  loop {
    let mut input_text = String::new();
    std::io::stdin()
      .read_line(&mut input_text)
      .expect("Failed to read input");

    if input_text.trim() == "q" {
      break;
    }

    let operators = vec!["+", "-", "/", "*"];

    for op in operators {

      match input_text.find(op) {
        Some(_) => {
          let parts: Vec<&str> = input_text
          .split(op)
          .collect();

          if parts.len() < 2 {
            panic!("Enter two numbers!");
          }

          let x:i64 = parts[0].trim().parse().ok().expect("Enter a number!");
          let y: i64 = parts[1].trim().parse().ok().expect("Enter a number!");

          match op {
            "+" => println!("{}", calc::add(x, y)),
            "-" => println!("{}", calc::sub(x, y)),
            "/" => println!("{}", calc::div(x, y)),
            "*" => println!("{}", calc::mul(x, y)),
            _ => println!("Sorry, only addition, subtraction, multiplication and division are supported")
          };

        },
        None => {}
      };
        
    }
}
}

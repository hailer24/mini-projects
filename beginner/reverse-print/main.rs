use std::io;


fn reverse<'a>(output: &'a mut String, input: &String, n: i32) -> &'a mut String {
  if n == 0 {
    return output;
  }

  output.push(input.chars().nth((n - 1) as usize).unwrap());
  reverse(output, input, n - 1)
}

fn main() {
  println!("Enter some text:");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
  input = input.trim().to_string();

  let mut output = String::new();
  println!("{}", reverse(&mut output, &input, input.len() as i32));
}
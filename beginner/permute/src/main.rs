use std::io;

fn permutations(word: String) -> Vec<String> {
    let length = word.len();
  
    if length <= 1 {
    
        return vec![word];
    }
    let trimmed = word.chars().skip(1).collect();
    let perms = permutations(trimmed);
    let current_char = word.chars().nth(0).unwrap();
    let mut result = Vec::new();
    for perm in &perms {
        for i in 0..&perms.len() + 1 {
        let front: String = perm.chars().take(i).collect();
        let rest: String = perm.chars().skip(i).collect();
        result.push(format!("{}{}{}", front, current_char, rest));
    }
  }
  result
}

fn main() {
    println!("Enter a word (max 6 letters): ");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input");

    input_text = input_text.trim().to_string();
    let len = input_text.len();
    if len > 6 {
      panic!("Input too long!");
    }

    let total_perms = permutations(input_text);
    let mut unique_perms = total_perms.clone();
    unique_perms.sort();
    unique_perms.dedup();
    println!("Permutations: \n{:?}\n", total_perms);
    println!("Distinct permutations: \n{:?}\n", unique_perms);
    println!("Total permutations: {}", total_perms.len());
    println!("Distinct permutations: {}", unique_perms.len());
}

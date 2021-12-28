use std::io;

fn permutations(temp:String) -> Vec<String> {
    let len  = temp.len();

    if len <= 1 { return vec![temp]; }
    
    let _temp = temp.chars().skip(1).collect();
    let perms = permutations(_temp);
    
    let curr = temp.chars().nth(0).unwrap();

    let mut ret: Vec<String> = Vec::new();
    for perm in &perms {
        for i in  0..&perm.len()+1 {
            let front: String = perm.chars().take(i).collect();
            let back: String = perm.chars().skip(i).collect();
            ret.push( format!("{}{}{}", front,curr,back));
        }
    }

    ret
    
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    input = String::from(input.trim());
    println!("{:?}", permutations(input));
}

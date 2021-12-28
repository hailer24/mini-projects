use std::io;

fn main() {
 let mut str:String = String::new();
 println!("Enter a string");
 io::stdin().read_line(&mut str).expect("failes to read input");
 str = String::from(str.trim());
 let mut c = 0;
 let mut v = 0;
 for i in str.chars() {
     if i.is_alphabetic() {
     match i {
         'a' | 'e' | 'i' |'o' | 'u' => v =v+ 1,
         _ => c = c+1
     };
    }
 }
 println!("{} consonents and {} vowels", c, v);
}

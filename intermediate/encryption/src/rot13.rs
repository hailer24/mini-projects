pub fn rot13(line: &String) ->String {

    let mut ret = String::new();
    for letter in line.chars() {
        if letter.is_alphabetic() {
            if letter.is_uppercase() {
                let mut temp = letter as u8;
                temp = (temp - ('A' as u8) + 13)%26 + ('A' as u8);
                ret.push(temp as char);
            }
            else {
                let mut temp = letter as u8;
                temp = (temp - ('a' as u8) + 13)%26 + ('a' as u8);
                ret.push(temp as char);
            }
        }
        else {ret.push(letter);}
    }
    ret
    }

#[test]
fn converts() {
    let input = "Hello World!".to_string();
  let converted = "Uryyb Jbeyq!".to_string();
  assert!(rot13(&input) == "Uryyb Jbeyq!");
  assert!(rot13(&converted) == "Hello World!");
}
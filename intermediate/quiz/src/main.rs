use serde_derive::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Read};

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
struct Question{
    question:String,
    answer: String
}
// fn to open a file

fn open_file(filename: String) -> String{
    let mut inp = String::new();
    let mut file = OpenOptions::new().read(true).open(filename).expect("Failed to find file");
    file.read_to_string(&mut inp).unwrap();
    inp
}

fn get_input() ->String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    String::from(buf.trim())
}

fn present(questions:Vec<Question>) {
    let mut score  = 0i64;
    for que in questions {
        println!("{}", que.question);
        if que.answer.to_lowercase() == get_input().to_lowercase() {
            println!("Correct Answer");
            score = score + 10;
        }
        else {
            println!("Wrong Answer");
        }
    }
    println!("Final Score{}", score);
}


fn main()  {
    let  data = open_file(String::from("./data.json"));
    let questions:Vec<Question> = serde_json::from_str(&data).unwrap();
    present(questions);
    
}
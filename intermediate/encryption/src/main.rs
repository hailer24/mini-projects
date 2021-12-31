use std::{fs::OpenOptions, io::{Read, Write}};
mod rot13;

fn main() -> std::io::Result<()> {
    let mut content = String::new();
    let mut f = OpenOptions::new().read(true).open("./secret.txt").unwrap();
    f.read_to_string(&mut content).unwrap();
    f = OpenOptions::new().read(true).write(true).truncate(true).open("./secret.txt").unwrap();
    f.write_all(rot13::rot13(&content).as_bytes()).expect("msg");
    Ok(())
}

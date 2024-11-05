use std::fs::File;
use std::io::{self, Read};
fn main() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    let username = read_username_from_file().expect("Failed to read username from file");
    println!("Username: {}", username);
}
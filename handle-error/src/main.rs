use std::fs::File;
use std::io::{self, Read};

fn main() {
    // let greeting_file = File::open("./hello.txt")
    //     .expect("hello.txt file should include");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();
    let mut username_file = File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

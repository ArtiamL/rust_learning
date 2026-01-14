#![allow(unused_variables, dead_code, clippy::question_mark)]
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // unwrap() creates a panic!() call automatically.
    let greeting_file_unwrap = File::open("unwrap.txt").unwrap();

    // expect() allows for custom panic!() error messages.
    let greeting_file_expect =
        File::open("expect.txt").expect("expect.txt should be included in this project!");
}

// Error propogation, long-way
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Error propogation, using ? shortcut
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Error propogation with chain operation on ? shortcut
fn read_username_from_file_chain() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("username_chain.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// fs::read_to_string, shortest way to read from file. Has same error handling &
// propogation as previous functions.
fn read_username_from_file_fs() -> Result<String, io::Error> {
    fs::read_to_string("username_fs.txt")
}

use std::{fs, fs::File};
use std::io::ErrorKind;
use std::io::{self, Read};
// RUST_BACKTRACE=1 cargo run
fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };

    // Same as above but with closures
    let greeting_file = File::open("hello.txt")
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt")
                    .unwrap_or_else(|error| {
                        panic!("Problem creating the file: {error:?}")
                    })
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        });
    
    // Auto-error on failure
    let greeting_file = File::open("hello.txt")
        .unwrap();

    // Choose the panic! error message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    /*
        In production-quality code, most Rustaceans choose
        expect rather than unwrap and give more context
        about why the operation is expected to always
        succeed. That way, if your assumptions are ever
        proven wrong, you have more information to use
        in debugging.
     */

    //  cannot use the `?` operator in a function that returns `()`
    // let greeting_file = File::open("hello.txt")?;
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

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

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?
        .read_to_string(&mut username)?;
    
    Ok(username)
}

fn read_username_from_file_shortestest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines()
        .next()?
        .chars()
        .last()
}
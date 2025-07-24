use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    println!("Hello World!");

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.text") {
    //             Ok(fc) => fc,
    //             Err(err) => panic!("Problem with creating the file{err:?}"),
    //         },
    //         _ => panic!("Problem with opening the file {error:?}"),
    //     },
    // };
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|err| {
    //             panic!("Problem with creating the file{err:?}");
    //         })
    //     } else {
    //         panic!("Problem with opening the file{error:?}");
    //     }
    // });
    //
    //
    let greeting_file_expect =
        File::open("hello.txt").expect("The hello.txt should be in the file");
    let greeting_file = File::open("hello.txt").unwrap();

    println!("{:?}", greeting_file);
}

fn read_user() -> Result<String, io::Error> {
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

fn read_username() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_short() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

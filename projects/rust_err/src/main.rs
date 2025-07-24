use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    let greeting_file_result = File::open("help.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(e) => match File::create("help.txt") {
            Ok(fs) => fs,
            Err(err) => panic!("Could not open or create the file{:?}", err),
        },
    };

    let mut s = String::new();
    match greeting_file.read_to_string(&mut s) {
        Ok(_) => (),
        Err(e) => panic!("Could not open file {:?}", e),
    }

    let mut greetings = File::open("help.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("help.txt").unwrap()
        } else {
            panic!("File should be avaialable");
        }
    });

    // File::open("help.txt")?.read_to_string(&mut s);
    fs::read_to_string(&mut s);
}

fn read_user() -> Result<String, io::Error> {
    let greeting_file_result = File::open("help.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(e) => match File::create("help.txt") {
            Ok(fs) => fs,
            Err(err) => return Err(err),
        },
    };

    let mut s = String::new();
    match greeting_file.read_to_string(&mut s) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    Ok(s)
}

fn read_user2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("help.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

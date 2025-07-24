use std::io;

fn main() {
    // TODO: Change the line below to fix the compiler error.
    let x: i32;

    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Wrong Intake");
        match input.trim().parse() {
            Ok(num) => {
                x = num;
                break;
            }
            Err(_) => continue,
        };
    }

    println!("Number {x}");
}

use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guest the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Input the number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fail to compile");
        println!("Your Guess - {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

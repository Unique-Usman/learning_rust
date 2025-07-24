#[derive(Debug)]

enum UsState {
    Alabame,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("Hello World {}", max);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

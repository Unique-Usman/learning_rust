#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum Option<T> {
//     Some(T),
//     None,
// }
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    let en = IpAddrKind::V4(String::from("127.0.0.1"));
    println!("{:?}", en);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 3, y: 3 };
    let m3 = Message::Write("usman".to_string());
    let m4 = Message::ChangeColor(3, 3, 4);

    println!("{m1:?} {m2:?} {m3:?} {m4:?}");

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    } else {
        ()
    }

    while let Some(max) = config_max {}
    let Some(max) = config_max else {
        return ();
    };
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coins: Coin) -> u8 {
    match coins {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Quarter => 15,
        Coin::Dime => 25,
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Option::Some(val) => Some(val + 1),
        Option::None => None,
    }
}

fn main() {
    let config_max: Option<i32> = Some(100);

    match config_max {
        Some(num) => println!("{}", num),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("{}", max);
    }
}

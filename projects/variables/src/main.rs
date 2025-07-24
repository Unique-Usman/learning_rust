fn main() {
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);

    tup.0 = 44;

    println!("{}", tup.0);
}


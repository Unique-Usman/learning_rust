fn main() {
    let a: [i32; 4] = [3, 3, 4, 5];

    for i in a {
        println!("{i}");
    }

    for i in 0..a.len() {
        println!("{}", a[i]);
    }
}

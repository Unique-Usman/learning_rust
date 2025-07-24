fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    for c in hello.bytes() {
        println!("{}", c);
    }
}

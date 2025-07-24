use ::std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);
    scores.entry(String::from("Blue")).or_insert(20);
    scores.entry(String::from("Yello")).or_insert(20);

    println!("{:?}", scores);
}

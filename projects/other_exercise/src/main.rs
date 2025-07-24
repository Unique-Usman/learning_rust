use std::collections::HashMap;

fn main() {
    let mut list = [2, 4, 4, 5, 54, 3];

    println!("{:?}", stats(&mut list));

    let apple = "fish";

    if let Some(answer) = convert_to_piglating(&apple) {
        println!("{}", answer);
    };
}

fn stats(list: &mut [i32]) -> (i32, i32) {
    list.sort();

    let size = list.len();

    let mid = size / 2;
    let mid = list[0];

    let mut temp: HashMap<i32, i32> = HashMap::new();
    for i in list {
        let count = temp.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut md_c = 0;

    for (key, value) in &temp {
        if *value > md_c {
            mode = *key;
            md_c = *value;
        }
    }

    return (mid, mode);
}

fn convert_to_piglating(data: &str) -> Option<String> {
    if let Some(first_char) = data.chars().nth(0) {
        let lower = first_char.to_ascii_lowercase();

        if "aeiou".contains(lower) {
            Some(String::from(data) + ".hay")
        } else {
            Some(String::from(&data[1..]) + "." + &first_char.to_string() + "ay")
        }
    } else {
        return None;
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//
//     let result = longest(string1.as_str(), string2);
//
//     println!("The longest string is {result}");
//
//     let novel = String::from("Call. me");
//     let first_word = novel.split(".").next().unwrap();
//
//     let i = ImportantExcerpt { part: first_word };
// }

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = "usman";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {result}");
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

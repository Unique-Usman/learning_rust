// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => return format!("{}{}", first.to_uppercase(), &input[1..]),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
    let vec: Vec<String> = words.iter().map(|x| capitalize_first(x)).collect();
    vec
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
    let vec: Vec<String> = words
        .iter()
        .map(|x| capitalize_first(x))
        .filter(|x| x != " ")
        .collect();
    // .fold(format!(""), |acc, x| format!("{acc} {x}"));
    vec.join(" ")
}

fn main() {
    // You can optionally experiment here.
    test_success();
    test_empty();
    test_iterate_string_vec();
    test_iterate_into_string();
}

fn test_success() {
    assert_eq!(capitalize_first("hello"), "Hello");
}

fn test_empty() {
    assert_eq!(capitalize_first(""), "");
}

fn test_iterate_string_vec() {
    let words = vec!["hello", "world"];
    assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
}
fn test_iterate_into_string() {
    let words = vec!["hello", " ", "world"];
    assert_eq!(capitalize_words_string(&words), "Hello World");
}

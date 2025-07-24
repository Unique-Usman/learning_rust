fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

fn largest<T: PartialOrd + Copy>(arr: &[T]) -> T {
    let mut largest = arr[0];

    for &ele in arr {
        if largest > ele {
            largest = ele
        }
    }

    largest
}

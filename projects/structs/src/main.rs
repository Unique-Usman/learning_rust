struct User {
    name: String,
    age: i32,
    height: i8,
    email: String,
}

struct Color(i32, i32, i32);

fn main() {
    let user: User = User {
        name: String::from("Usman"),
        age: 23,
        height: 23,
        email: String::from("usman@"),
    };

    let black = Color(0, 0, 0);

    println!(
        "{} age is {} with height {} and email {}",
        user.name, user.age, user.height, user.email
    );

    println!("{}", black.0);
    println!("{}", black.1);
    println!("{}", black.2);
}

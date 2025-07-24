fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(42);
    v.push(332);

    for i in 0..2 {
        println!("{}", v[i]);
    }

    let v2: Vec<i32> = vec![1, 2, 3];

    let temp = v2[2];
    println!("{}", temp);

    let third: &i32 = &v2[2];
    println!("{}", third);

    let third: Option<&i32> = v2.get(2);

    if let Some(value) = third {
        println!("The third eleement is {value}");
    } else {
        println!("There is not third element");
    }

    let mut v3: Vec<i32> = vec![1, 2, 3];

    println!("===============================");

    for i in &mut v3 {
        *i += 1;
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("Usman")),
    ];
}

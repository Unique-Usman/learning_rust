struct CustomPointer {
    data: String,
}

impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("Droping the data: {}", self.data);
    }
}

fn main() {
    let c = CustomPointer {
        data: String::from("Hello"),
    };

    let d = CustomPointer {
        data: String::from("Alaye"),
    };
    println!("The end");
}

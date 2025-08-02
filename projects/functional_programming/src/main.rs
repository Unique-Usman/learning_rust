fn main() {
    // let mut fs;
    // {
    //     let a: String = String::from("Usman");
    //     fs = move |c: String| -> String {
    //         println!("Printing the element inside {}", a);
    //         return c;
    //     };
    // }
    //
    // var(fs, String::from(""))

    // let vec = vec![1, 2, 3, 4];
    //
    // for v in &vec {
    //     println!("{v}");
    // }
    //
    // for v in vec.iter() {
    //     println!("{v}");
    // }
    //
    // for v in &vec {
    //     println!("{v}");
    // }
    //
    // let v2: Vec<&u32> = vec.iter().collect();
    // let m = Mine { ele: 2 };
    // m.iter().map(|x| x + 1);

    let v = vec![2, 4, 5];

    // for a in v {
    //     println!("{a}");
    // }
}

// // fn var(mut f: impl FnOnce(String) -> String, s: String) {
// //     f(s);
// // }
//
// fn var<T: FnOnce(String) -> String>(f: T, s: String)
// // where
// //     T: FnOnce(String) -> String,
// {
//     f(s);
// }

struct Mine {
    ele: u32,
}

impl Iterator for Mine {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ele > 0 {
            self.ele -= 1;
            Some(self.ele)
        } else {
            None
        }
    }
}


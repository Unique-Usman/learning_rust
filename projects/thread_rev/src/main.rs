use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    let t1 = thread::spawn(move || {
        let vec = vec![
            String::from("Usman"),
            String::from("is"),
            String::from("the"),
            String::from("goat"),
        ];

        for v in vec {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let t2 = thread::spawn(move || {
        let vec = vec![
            String::from("Usman"),
            String::from("is"),
            String::from("the"),
            String::from("goat"),
        ];
        for v in vec {
            tx2.send(v).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for rv in rx {
        println!("{}", rv);
        thread::sleep(Duration::from_millis(100));
    }

    t1.join().unwrap();
    t2.join().unwrap();
}

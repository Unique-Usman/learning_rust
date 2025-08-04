use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vec = vec![
            String::from("Usman"),
            String::from("is"),
            String::from("good"),
            String::from("boy"),
        ];

        for v in vec {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::spawn(move || {
        let vec = vec![
            String::from("Usman"),
            String::from("is"),
            String::from("good"),
            String::from("boy"),
        ];

        for v in vec {
            tx2.send(v).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    for rev in rx {
        println!("{rev}");
        thread::sleep(Duration::from_millis(10));
    }
}

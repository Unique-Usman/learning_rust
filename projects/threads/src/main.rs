use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let counter = 4;
    let t: JoinHandle<i32> = thread::spawn(move || {
        for i in 1..10 {
            println!("hi no {} from the spawned thread", counter);
            thread::sleep(Duration::from_millis(1));
        }

        counter
    });

    for i in 1..5 {
        println!("hi no {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    t.join().unwrap();
    // println!("Hello World from the outside thread");
}

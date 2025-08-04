use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut m = Arc::new(Mutex::new(42));
    let mut handles = vec![];

    for _ in 0..10 {
        let m = Arc::clone(&m);

        let handle = thread::spawn(move || {
            let mut num = m.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{m:?}");
}

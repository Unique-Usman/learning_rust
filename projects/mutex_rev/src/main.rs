use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let s: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![]));

    let s1 = Arc::clone(&s);
    let s2 = Arc::clone(&s);

    let t1 = thread::spawn(move || {
        s1.lock().unwrap().push(42);
    });

    let t2 = thread::spawn(move || {
        s2.lock().unwrap().push(42);
    });

    t1.join().unwrap();
    t2.join().unwrap();
    for v in s.lock().unwrap().iter() {
        println!("{}", v);
    }
}

// use std::cell::RefCell;
//
// struct Counter {
//     count: RefCell<u32>,
// }
//
// impl Counter {
//     fn add(&self) {
//         *self.count.borrow_mut() += 1;
//     }
//
//     fn get(&self) -> u32 {
//         *self.count.borrow()
//     }
// }
// fn main() {
//     let counter = Counter {
//         count: RefCell::new(0),
//     };
//
//     counter.add();
// //     println!("Count is: {}", counter.get());
// //     counter.add();
// // // }
// //
// // use std::cell::RefCell;
// // use std::rc::Rc;
// //
// // struct Counter {
// //     value: Rc<RefCell<u32>>,
// // }
// //
// // impl Counter {
// //     fn increment(&self) {
// //         *self.value.borrow_mut() += 1;
// //     }
// //
// //     fn get(&self) -> u32 {
// //         *self.value.borrow()
// //     }
// //
// //     fn new(value: u32) -> Self {
// //         Counter {
// //             value: Rc::new(RefCell::new(value)),
// //         }
// //     }
// // }
// //
// // fn main() {
// //     let share_value = Rc::new(RefCell::new(0));
// //
// //     // let a = Counter {
// //     //     value: Rc::clone(&share_value),
// //     // };
// //     //
// //     // let b = Counter {
// //     //     value: Rc::clone(&share_value),
// //     // };
// //
// //     let c = RefCell::new(Counter::new(0));
// //     // let b = c.borrow_mut();
// //     // *b.value.borrow_mut() += 1;
// //
// //     // a.increment();
// //     // b.increment();
// //
// //     // println!("Shared count: {}", c.get());
// // }
//
// use std::cell::RefCell;
//
// struct Cache {
//     value: RefCell<Option<String>>,
// }
//
// impl Cache {
//     fn get(&self) -> String {
//         let mut val = self.value.borrow_mut();
//
//         if val.is_none() {
//             println!("Computing value...");
//             *val = Some("Expensive result".to_string());
//         }
//
//         val.as_ref().unwrap().clone()
//     }
// }
//
// fn main() {
//     let cache = Cache {
//         value: RefCell::new(None),
//     };
//
//     println!("First call: {}", cache.get());
//     println!("Second call: {}", cache.get());
// // }
//
// use std::cell::RefCell;
// use std::rc::Rc;
//
// trait Subscriber {
//     fn notify(&self, msg: &str);
// }
//
// struct Logger {
//     logs: RefCell<Vec<String>>,
// }
//
// impl Logger {
//     fn new() -> Self {
//         Self {
//             logs: RefCell::new(vec![]),
//         }
//     }
//
//     fn show(&self) {
//         for log in self.logs.borrow().iter() {
//             println!("LOG: {}", log);
//         }
//     }
// }
//
// impl Subscriber for Logger {
//     fn notify(&self, msg: &str) {
//         self.logs.borrow_mut().push(msg.to_string());
//     }
// }
//
// struct Publisher {
//     subs: Vec<Rc<dyn Subscriber>>,
// }
//
// impl Publisher {
//     fn new() -> Self {
//         Self { subs: vec![] }
//     }
//
//     fn subscribe(&mut self, sub: Rc<dyn Subscriber>) {
//         self.subs.push(sub);
//     }
//
//     fn broadcast(&self, msg: &str) {
//         for sub in &self.subs {
//             sub.notify(msg);
//         }
//     }
// }
//
// fn main() {
//     let logger = Rc::new(Logger::new());
//
//     let mut pubsub = Publisher::new();
//     pubsub.subscribe(Rc::clone(&logger));
//
//     pubsub.broadcast("Message 1");
//     pubsub.broadcast("Message 2");
//
//     logger.show(); // See both messages
// // }
//
// pub trait Messenger {
//     fn send(&self, msg: &str);
// }

use std::rc::Rc;

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;

fn main() {
    let val = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    let b = Cons(Rc::clone(&val), Rc::clone(&a));

    *val.borrow_mut() = 3;
}

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

struct LiveTracker;

struct Con {
    vec: RefCell<Vec<String>>,
}

impl Messenger for Con {
    fn send_message(&self, mess: String) {
        self.vec.borrow_mut().push(mess);
    }
}

trait Messenger {
    fn send_message(&self, mess: String);
}

impl LiveTracker {
    fn send(&self, mes: String, mess: &impl Messenger) {
        mess.send_message(mes);
    }
}

#[derive(Debug)]
struct Mybox<T> {
    data: T,
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

impl<T> Mybox<T> {
    fn new(value: T) -> Self {
        Mybox { data: value }
    }
}
impl<T> Deref for Mybox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Mybox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn print(value: &str) {
    println!("Omoo - You gats learn Rust Language ooo {value}");
}

fn main() {
    let y = 32;
    let mut c = Mybox::new(y);
    // *c = 4;
    assert_eq!(*c, y);

    let str = Mybox::new(String::from("Rust"));
    print(&str);
    print(&((*str)[..]));

    let l = Cons(4, Box::new(Nil));

    let a = Rc::new(Node {
        value: 4,
        next: None,
    });

    let b = Node {
        value: 32,
        next: Some(Rc::clone(&a)),
    };

    let mut c = Node {
        value: 5,
        next: Some(Rc::clone(&a)),
    };

    println!("a- {a:?}");
    println!("b- {b:?}");
    println!("c- {c:?}");

    c.next = (Some(Rc::new(b)));
    println!("c- {c:?}");

    let b = Con {
        vec: RefCell::new(vec![]),
    };

    let live = LiveTracker;

    live.send(String::from("Usman"), &b);
    println!("{}", b.vec.borrow().len());
}

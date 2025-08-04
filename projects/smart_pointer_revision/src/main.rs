use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

struct MyBox<T> {
    data: T,
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> MyBox<T> {
    fn new(data: T) -> Self {
        MyBox { data }
    }
}

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
#[derive(Debug)]
struct Node {
    value: i32,
    previous: Option<Weak<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

fn print(s: &str) {
    println!("This is the element in the String {}", s);
}

fn main() {
    let a = Cons(3, Box::new(Nil));
    let b = Cons(3, Box::new(a));

    println!("{b:?}");

    print(&String::from("Usman Akinyemi"));

    let mut c = Rc::new(RefCell::new(Node {
        value: 54,
        previous: None,
        next: None,
    }));
    println!("c = {:?}", c);
    println!("{}", Rc::strong_count(&c));

    let b = Rc::new(RefCell::new(Node {
        value: 56,
        previous: None,
        next: Some(Rc::clone(&c)),
    }));

    c.borrow_mut().previous = Some(Rc::downgrade(&b));
    //

    println!("{}", Rc::strong_count(&c));
    println!("b = {:?}", b);
}

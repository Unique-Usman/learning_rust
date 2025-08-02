#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Reference count = {}", Rc::strong_count(&a));
    let c = Cons(3, Rc::clone(&a));
    println!("Reference count = {}", Rc::strong_count(&a));

    if let Cons(_, rc) = &c {
        println!("Reference count = {}", Rc::strong_count(rc));
        println!("Refer: {:?}", rc);
    }

    {
        let b = Cons(4, Rc::clone(&a));
        println!("Reference count = {}", Rc::strong_count(&a));
    }
    println!("Reference count = {}", Rc::strong_count(&a));
}

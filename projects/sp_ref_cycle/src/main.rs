use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum Node {
    Cons(i32, RefCell<Vec<Rc<Node>>>),
    Nil,
}

struct NodeList {
    value: i32,
    parent: RefCell<Weak<NodeList>>,
    children: RefCell<Vec<Rc<NodeList>>>,
}

use crate::Node::{Cons, Nil};

fn main() {
    // let a = Rc::new(Cons(32, RefCell::new(vec![])));
    // let b = Rc::new(Cons(2, RefCell::new(vec![Rc::clone(&a)])));
    // if let Cons(_, vec_cell) = &*a {
    //     vec_cell.borrow_mut().push(Rc::clone(&b));
    // }
    //
    // println!("a {:?}", b);

    let leaf = Rc::new(NodeList {
        value: 43,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(NodeList {
        value: 43,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}

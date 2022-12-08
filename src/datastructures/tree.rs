use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node<T> {
    parent: RefCell<Option<Weak<Node<T>>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
    elem: T,
}

pub struct Tree<T> {
    root: Option<Rc<Node<T>>>,
}



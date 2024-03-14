// cons list:
//  - comes from Lisp
//  - the cons function ("constructive function") constructs a new pair from its two args
//  - each pair is usually a single value + another pair (pairs containing pairs containing pairs...)
//  - similar to linked list, each node has value then a pointer to another node
//  - last node contains a value called Nil instead of a node

use std::rc::Rc;
use std::cell::RefCell;

pub enum List<T> {
    Cons(T, Box<List<T>>), // recursive case
    Nil // base case
}

pub enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Nil
}

// RefCell allos us to mutate immutable T's in the list (unsafely)
#[derive(Debug)]
pub enum PowerList<T> {
    Cons(Rc<RefCell<T>>, Rc<PowerList<T>>),
    Nil
}

#[derive(Debug)]
pub enum DangerList {
    Cons(i32, RefCell<Rc<DangerList>>),
    Nil
}

impl DangerList {

    pub fn next(&self) -> Option<&RefCell<Rc<DangerList>>> {
        match self {
            DangerList::Cons(_, next) => Some(next),
            DangerList::Nil => None,
        }
    }

}
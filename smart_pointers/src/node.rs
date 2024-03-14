use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    // RefCell : allows mutation of the child vector (detach/reattach)
    // Rc: allows shared immutable pointers to specific nodes
    pub children: RefCell<Vec<Rc<Node>>>,
    // to avoid cycle, use a weak ref to the parent
    // weak =~ "I point but never own"
    pub parent: RefCell<Weak<Node>>,
}
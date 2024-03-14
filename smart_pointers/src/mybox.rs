use std::ops::Deref;
use std::ops::DerefMut;

// tuple struct with 1 element
pub struct MyBox<T>(T);

impl<T> MyBox<T> {

    pub fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }

}

// impl <Trait> for <Struct>
impl<T> Deref for MyBox<T> {

    // "associative type"
    type Target = T;

    // deref is effectively the overload for the '*' operator
    // return a reference to the value you want the '*' operator to return
    fn deref(&self) -> &Self::Target {
        // MyBox is a tuple struct; self.0 is the first value
        &self.0
    }
}

// the MutDeref trait is needed to support the '*' operator for a mutalbe reference
// this inherits Target from Deref
impl<T> DerefMut for MyBox<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for MyBox<T> {

    fn drop(&mut self) {
        println!("Dropping box.")
    }
}

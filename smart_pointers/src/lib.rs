// RefCell<T> is like a Box<T> (single ref)
// provides "interior mutability"
// - allows you to mutate data even when there are other references in the same scope !!!
// - requires "unsafe" code inside a struct impl fn
// - mutable data is private; can only be mutated via a public method
// downsides:
// - RefCell<T> has no compile time validation, may crash at runtime
// - single thread case only

pub mod cons;
pub mod mybox;
pub mod quota;
pub mod node;

#[cfg(test)]
mod tests {

    // Ref<T> is a smart pointer around an immutable ref
    // - implements Deref : can be treated like a regular ref in code
    // - implements Drop : parent object's ref count is decremented by 1 when it goes out of scope
    
    // MutRef<T> is a smart pointer around a mutable ref
    // - implements Deref : can be treated like a regular mut ref in code
    // - implements Drop : parent object's ref count is decremented by 1 when it goes out of scope

    // RefCell<T>
    // .borrow() - gets an immutable reference to the value : Ref<T>
    // .borrow_mut() - gets a mutable reference to the value : MutRef<T>
    // RefCell<T> maintains a counter for both immutable and mutable borrows.
    // RefCell<T> allows to have 1 mutable borrow at a time, or N immutable borrows.
    // however this is not checked at compile time!  RefCell will panic at runtime instead.
    use std::cell::RefCell;
    use super::quota::Quota;
    use super::quota::Messenger;

    // Stub implementation of the Messenger trait
    struct MessengerStub {
        //messages: Vec<String>,
        messages: RefCell<Vec<String>>,
    }

    impl MessengerStub {
        fn new() -> MessengerStub {
            MessengerStub {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MessengerStub {
        fn send(&self, msg: &str) {
            // problem: Messenger trait defines send(&self) with immutable self reference
            // in order for our Stub to record the sent messages,
            // we need to use the "interior mutability" pattern
            //self.messages.push(String::from(msg));
            self.messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_90_percent_warning_message() 
    {
        let mock_messenger = MessengerStub::new();
        let mut limit_tracker = Quota::new(&mock_messenger, 100);

        limit_tracker.set_value(91);

        assert_eq!(mock_messenger.messages.borrow().len(), 1);
    }

    #[test]
    #[should_panic]
    fn it_panics()
    {
        let r = RefCell::new(45);
        let a = r.borrow_mut();
        let b = r.borrow(); // should panic
        let _c = *a + *b;
    }
}
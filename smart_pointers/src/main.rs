// smart pointers are structs that implement the Deref and Drop traits.
// Deref : allows the struct to behave like a reference, support the '*' operator, and support "reference coercion"
// Drop : allows you to add a teardown method, called when an instance goes out of scope.

// most common types:
// Box<T> : allocated on the heap (rather than the stack)
// Rc<T> : a reference counting pointer, can have multiple ownership
// Ref<T> : enforces borrowing rules at runtime insteado f compile time

use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use smart_pointers::mybox::MyBox;
use smart_pointers::cons::List::{Cons, Nil};
use smart_pointers::cons::RcList;
use smart_pointers::cons::PowerList;
use smart_pointers::cons::DangerList;
use smart_pointers::node::Node;

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {

/*
    // Boxes are often used when you only care that a type implements a specific trait (interface)
    // the specific object type is unknown at compile time
    // this is known as a "trait object"
    let b = Box::new(5);
    println!("b = {}", b);

    // Rust needs to know the size of a type at compile time
    // this can't be done for "recursive" structs like cons list
    // unless you use pointers in the struct definition, like Box<T>
    // the size of Box<T> is always known (usize = size of 1 pointer)
    // Box::new provides a heap allocation
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // dereference operator
    {
        let x = 5;
        let y = Box::new(5);
        assert_eq!(5, *y); // dereferencing a box explicitly
        //assert_eq!(5, y); // can't compare T == Box<T>

        let z = MyBox::new(5);
        assert_eq!(5, *z); // works now that we implemented Deref
        // behind the scenes, (*z) compiles as *(z.deref())
    }

    // deref corecion
    // converts a reference to type X to a reference of type Y
    {
        hello("str slice");
        hello(&String::from("deref coercion"));

        // de ref coercion works for boxed refs also
        // At compile time, Rust will call ::deref() on the parameter, until it gets a reference which matches the function arg type (&str)
        // so Rust calls MyBox::deref() -> &String, then String::deref() -> &str
        let boxed = MyBox::new(String::from("boxed String"));
        hello(&boxed);

        // explict version (doesn't take advantage of deref coercion)
        let r = &(*boxed);
        let r = &r[..];
        hello(r);
    }

    // mutable
    {
        let mut x = MyBox::new(5);
        *x = 7;
        assert_eq!(7, *x);

        // mutable refs can be coerced into immutable refs, but not the other way around
        let mut y = MyBox::new(String::from("hamsters"));
        hello(&y);
    }

    // ref dropping
    {
        println!("-- creating box.");
        let x = MyBox::new("asdf");
        println!("-- dropping box.");
        // x.drop() // illegal
        // drop accepts a struct as parameter (takes ownership of the struct being dropped)
        std::mem::drop(x);
        println!("-- end.");
    }
*/
    // Rc<T> : referenc counting
    // only works for immutable reference sharing in a single thread
    // useful for graphing type problems
    /*{
        use RcList::{Cons, Nil};

        let a = Cons(4, Rc::new(Cons(-3, Rc::new(Cons(1, Rc::new(Nil))))));

        let shared = Rc::new(a);
        println!("count after creating a = {}", Rc::strong_count(&shared));

        // Rc::clone increments the reference count

        let b = Cons(5, Rc::clone(&shared));
        println!("count after creating b = {}", Rc::strong_count(&shared));
        {
            let c = Cons(6, Rc::clone(&shared));
            // b and c are now cons lists which share an intermediate list of nodes
            println!("count after creating c = {}", Rc::strong_count(&shared));
        }

        // Rc::drop decrements the reference count

        println!("count after dropping c = {}", Rc::strong_count(&shared));
    }*/

    /*
    {
        use PowerList::{Cons, Nil};

        let value = Rc::new(RefCell::new(5));

        // shared piece of list (Rc)
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        
        // unshared lists
        let b = Cons(Rc::new(RefCell::new(9)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(0)), Rc::clone(&a));

        // we can mutate value even though it's immutabl ref is shared by b and c
        // note about reference coercion:
        // here value::deref is called repeatedly (by the compiler)
        // until we get a &RefCell since this is the &self argument to borrow_mut
        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }*/

    // Rust is vulnerable to "memory cycles"
    // If you have two Rc's that can point to eachother (A contains Rc<B>, B contains Rc<A>)
    // both of their "strong_counts" will never reach zero (which is necessary for deallocation)
    // the program won't free the memory before exit!
    /*
    {
        use DangerList::{Cons, Nil};
        // a: 5 -> Nil
        let n1 = Cons(5, RefCell::new(Rc::new(Nil)));
        let a = Rc::new(n1);

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.next());

        // b: 10 -> a -> Nil
        let n2 = Cons(10, RefCell::new(Rc::clone(&a)));
        let b= Rc::new(n2);

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.next());

        // b:10 -> a:5 -> b:10 -> ...
        if let Some(node) = a.next() {
            let mut next = node.borrow_mut();
            *next = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // this causes a memory leak
        // a and b have both been cloned once => ref count is 2
        // when b goes out of scope, ref count of b will be decreased by 1
        // when a goes out of scope, ref count of a will be decreased by 1
        // neither reaches zero
        //println!("a next item = {:?}", a.next());
    } */

    {
        // call Rc::downgrade to get a weak reference to a shared value
        let mut r = Rc::new(5);
        let b = Rc::downgrade(&r);
        println!("strong count: {}, weak count: {}", Rc::strong_count(&r), Rc::weak_count(&r));

        // to get a value from a weak pointer, call upgrade()
        if let Some(rc) = b.upgrade() {
            println!("you have {} dollars", *rc);
        }

        // r will be cleaned up even if the weak_count > 0

        std::mem::drop(r);

        match b.upgrade() {
            Some(rc) => println!("you still have {} dollars", *rc),
            None => println!("wallet unavailable")
        };
    }

    {
        let leaf = Rc::new(Node {
            value: 3,
            children : RefCell::new(vec![]),
            parent : RefCell::new(Weak::new()),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        {
            let branch = Rc::new(Node {
                value: 8,
                children: RefCell::new(vec![Rc::clone(&leaf)]),
                parent: RefCell::new(Weak::new()),
            });

            // assign branch as parent of leaf
            {
                let mut weakRef = leaf.parent.borrow_mut();
                *weakRef = Rc::downgrade(&branch);
            }

            println!(
                "branch refs : strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );
    
            println!(
                "leaf refs : strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        // branch has gone out of scope
        // weakref didn't prevent it from being dropped
        // weakref.upgrade() now returns None
        println!(
            "leaf refs : strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
}

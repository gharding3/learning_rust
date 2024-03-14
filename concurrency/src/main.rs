// Rust concurrency
// - Rust std threads are 1:1 with OS threads
// - Virtual threads are provided by separate libraries
// "message passing"
// - actor model : only 1 thread can work on the data at a time.
//      => all messages have single ownership
// - std lib provides a "channel" for message sending
// "shared state"
// - 

use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, MutexGuard};

fn main() {
    demo_mutex();
}

fn demo_mutex()
{
    // Mutex<T> wraps and takes ownership of an object
    // the wrapped object cannot be accessed without locking
    // Mutex<T> : lock() :: RefCell<T> : borrow_mut()
    let m:Mutex<i32> = Mutex::new(5);
    {
        // lock() will fail (Result::Err) if another thread holding the lock panics
        let mut num:MutexGuard<i32> = m.lock().unwrap();
        // MutexGuard implements Deref so we can use the *operator to change the wrapped value
        *num = 6;
        // MutexGuard implements Drop so that the lock is released as soon as it goes out-of-scope
    }
    println!("m = {:?}", m);

    // to share a mutex between multiple threads, wrap it in Arc<T> (same interface as Rc<T>)
    // recall Rc<T> is not safe to share across threads (its internal reference counter is not atomic)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // do some concurrent operations

            // get a lock around the final result to update it
            let mut counter_guard = counter.lock().unwrap();
            *counter_guard += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter = {:?}", *counter.lock().unwrap());
}

fn actor_model() {
    // mpsc: "multiple producer, single consumer"
    // returns a transmitter and receiver
    let (tx, rx) = mpsc::channel();

    // with mspc, the tx end can be cloned to set up multiple producers
    let tx1 = tx.clone();

    // use move closure to take ownership of tx
    // at the end of the closure, the tx is closed.
    // this closes the channel (a channel is closed when either tx or rx is dropped)
    thread::spawn(move || {
        let val = String::from("rock over london");
        tx.send(val).unwrap();
        let val = String::from("rock on chicago");
        tx.send(val).unwrap();
        // illegal, val is moved by send
        // as soon as we call send, we have to assume the receiver is the new owner
        //println!("val is {}", val); 
    });

    thread::spawn(move || {
        let val = String::from("wheaties");
        tx1.send(val).unwrap();
        let val = String::from("breakfast of champions");
        tx1.send(val).unwrap();
    });

    // retrieve from the main thread
    // recv() will block; try_recv() will not block
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // rx can also be treated like an iterator
    for received in rx {
        println!("Got: {}", received);
    }
}

fn simple_thread() {
    let handle = {
        let v = vec![1, 2, 3];

        let handle:JoinHandle<()> = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            println!("Here's a vector: {:?}", v);
        });

        handle
    };

    thread::sleep(Duration::from_millis(100));

    handle.join().unwrap();

    // at end of main, all unfinished threads will be stopped
}

/*
recoverable errors:
- Result<T, E>
enum Result<T, E> {
    Ok(T),
    Err(E),
}
unrecoverable errors:
- panic! macro
- by default, the program will unwind and clean up the stack
- you can change the bheavior to "abort" the program immediately instead,
relying on the OS to clean up the program memory.
*/

use std::{fs::File, io::{Error, ErrorKind, Read}};

pub fn to_panic_or_not_to_panic() -> Result<(), Box<dyn std::error::Error>> {

    // most of the time we should not panic
    //panic!("I crashed.");

    // better error handling : use Result
    let f : Result<File,Error> = File::open("hello.txt");
    let _f = match f {
        Ok(file) => { 
            println!("File already exists");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(new_file) => new_file,
                Err(e) => panic!("Error creating file: {:?}", e)
            }
            _ => panic!("File problem: {:?}", error)
        }
    };

    // simpler syntax for error-handling a Result<T,E>
    // something called "closures"
    let f : Result<File,Error> = File::open("world.txt");
    let _file = f.unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        }
        else {
            panic!("Crash and burn...");
        }
    });

    // .unwrap() is a shortcut to "give me the result T or panic"
    // .expect() does the same thing but has a custom error message (includes the Err details)
    //let f  = File::open("jello.txt").expect("Jello not found.");

    let s = read_username("hello.txt").unwrap();
    println!("username = {}", s);

    // use the ? operator to unwrap or return the Error result
    let f = File::open("hello world.txt")?;
    println!("file = {:?}", f);

    Ok(())
}

// propagating errors to the caller
// this can be achieved by returning a Result<T, E> from your method
// (there is no exception throwing in Rust)
fn _read_username_from_file(filename: &str) -> Result<String, Error> {
    let f = File::open(filename);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// implementation using ? operator
// use ? after a Result<T,E> expression
// if the Result is Ok, the value in the Ok enum is returned
// if the Result is Err, the Err will be returned immedatiely from the current function.
// ? can only be used inside a function which has a compatible return type (Result, Option, FromResidual)
// Option and Result are very similar...
// one has None as it's negative variant (with no data attached)
// reuslt has Err(error) with an error data attached
fn read_username(filename: &str) -> Result<String, Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    // Note: there is a std::fs::read_to_string static function which can be used
}
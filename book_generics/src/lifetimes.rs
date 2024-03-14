use std::fmt::Display;

/*
Every reference has a lifetime or scope
most of the time these are inferred by the compiler ("lifetime elision")
other times, Rust requires an annotation to deduce the lifetime.

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

*/

// Here Rust "borrow checker" doesn't know whether the reference of x or y is returned
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
*/
// Tells Rust that both x and y share some minimum lifetime 'a'
// the return value will have this minimum lifetime 'a'
// any invocation of the function must satisfy this "lifetime contract"
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// below is invalid; 
// if the return value is derived from arguments,
// the return type's lifetime ("output lifetime") must match the lifetime of those args ("input lifetime")
/* 
pub fn bad_lifetime<'a,'b>(x: &'a str, y: &'b str) -> &'b str {
    if x.len() > y.len() { x } else { y }
}
*/

// lifetime is not required for functions that have one input and one output
// Rust deduces that the lifetime is the same for both
pub fn first_word(x: &str) -> &str {
    x.split('.').next().unwrap()
}

// Using lifetimes we can create structs that hold references
pub struct Excerpt<'a> {
    pub text: &'a str
}

// use the impl<'a> syntax to implement a struct with lifetimes
impl<'a> Excerpt<'a> {
    // Rust deduces that str and Excerpt have the same lifetime
    pub fn from(slice : &str) -> Excerpt {
        Excerpt { text: slice }
    }
    // Rust deduces that the output lifetime
    // is the same as the lifetime of "self" 
    pub fn text(&self) -> &str {
        self.text
    }

    // lifetime mismatch:
    // default output lifetime for method with self is the struct lifetime
    // here we are returning a ref with a different lifetime
    /*pub fn echo(&self, slice: &str) -> &str {
        slice
    }*/
    // fixed version 1: give input and output parameter a lifetime independent of the struct
    pub fn echo1<'b>(&self, slice: &'b str) -> &'b str {
        slice
    }
    // fixed version 2: input and output must have the same lifetime as the struct
    pub fn echo2(&self, slice: &'a str) -> &'a str {
        slice
    }
}

impl Display for Excerpt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.text)
    }
}

// Special lifetime : 'static
// means it lives for duration of program
// not recommended
pub fn static_world() {
    // string literals implicitly have a static lifetime
    let s : &'static str = "I have a static lifetime";
    println!("{}", s);
}
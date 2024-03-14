use std::fmt::Display;


/*
 Traits are like interfaces in other languages
 Structs which implement the trait must fulfill the "behaviors" (method contract).
 */
pub trait Summary {
    fn summarize(&self) -> String;
    fn thumbnail(&self) -> String {
        // default implementation
        unimplemented!()
    }
}

pub struct NewsArticle {
    pub headline : String,
    pub author : String,
}

// To implement a trait, use this syntax: impl <trait> for <struct>
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

// You can implement traits on primitive types
// or any type as long as either type or the trait is defined in the local crate
// (you cannot implement external traits on external types)
impl Summary for i32 {
    fn summarize(&self) -> String {
        todo!() // panics with "not yet implemented"
        //unimplemented!() // panics with "not implemented"
    }
}

// You can also implement a trait conditionally for types that implement a different trait
// NOTE: The compiler will error if you have types which have multiple trait implementations
// so it's best to create a unique trait for each condition.
/*
impl<T: Display> Summary for T {
    fn summarize(&self) -> String {
        self.to_string()
    }
}
*/

// function which only accepts types that implement a trait
// need to use generics with constraint; Summary cannot be used as a type
pub fn printSummary<T: Summary>(s : &T) -> () {
    println!("{}", s.summarize());
}
// short form : use "impl" keyword
// this has exact same meaning as the constraint above
pub fn printSummary2(s: &impl Summary) {
    println!("{}", s.summarize());
}

// interesting feature: you can require multiple trait bounds on the same parameter
// here s must implement both the Summary and Display traits, or the compiler will fail
pub fn printSummary3(s: &(impl Summary + Display)) {
    println!("{}", s.summarize());
}

// another syntax
pub fn printSummary4<T: Summary + Display>(s : &T) {
    println!("{}", s.summarize());
}

// yet another equivalent syntax
pub fn printSummary5<T>(s : &T) 
    where T: Display + Summary
{
    println!("{}", s.summarize());
}

// traits can also be a return type, using the -> impl <trait> syntax
// however, this only works if your function has a single possible return type
pub fn returns_summary() -> impl Summary {
    NewsArticle { 
        headline: String::from("Lost Balloon Found"),
        author: String::from("Eggman")
    }
}
use std::error::Error;

mod errors;

fn main() -> Result<(), Box<dyn Error>>
{
    // execution will stop here if the fn returns an Error Result
    // app will terminate with a non-zero exit code (1)
    errors::to_panic_or_not_to_panic()?;

    // ternary operator
    let x = false;
    let z = if x { 1 + 1 } else { 1 + 2 };
    println!("z = {}", z);

    // expressions - similar to lambdas (anon functions)?
    let y:u16 = { 9 };
    let y = { let x = 2; y * x };
    println!("y = {}", y);

    let s1:String = String::from("yello");
    //let l1:usize = calculate_length(&s1);
    let s1ref = &s1;
    let anothers1ref = &s1;
    //let s1mutef = &mut s1;  cannot create mut ref from non-mut variable
    let l1 = (calculate_length(s1ref) + calculate_length(&anothers1ref)) / 2;

    let mut s2 = String::from("gex");
    // interestingly, we cannot have an immutable ref and a mutable ref in the same scope.
    // "immutable borrow" and "mutable borrow" cannot co-exist
    // by the same logic, we cannot have multiple mutable refs.
    //let s2ref = &s2;
    let s2mutref = &mut s2;
    //let anothers2mutref = &mut s2;
    modify_string(s2mutref);
    let l2 = calculate_length(s2mutref); // mutable ref can be cast to ref

    println!("length of '{}' is {}", s1, l1);
    println!("length of '{}' is {}", s2, l2);

    // when a main fn returns Ok, the app exits with exit code of 0
    return Ok(());
}

// & : reference an object without taking ownership of it
// rust calls this "borrowing"
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn modify_string(s: &mut String) {
    s.push_str("...");
}


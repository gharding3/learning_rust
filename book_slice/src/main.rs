fn main() 
{
    // array slices
    let a:[i32] = [1,2,3,4,5];
    let slice_of_a:&[i32] = &a[2..];
    println!("a = {:?}", slice_of_a);

    // Rust String slice type : &str

    // string literals are slices
    let pizzas:&str = "cheese pep olive ";

    // fn which accepts str& works on all of: string literals, string slices, and references of String objects
    let s = get_slice(pizzas, 0);
    println!("My slice: {}", s);

    let pizzaz = String::from(pizzas);

    let s = get_slice(&pizzaz, 2);
    println!("Your slice: {}", s);
}

fn get_slice(s: &str, n:usize) -> &str {
    assert!(usize >= 0, "a choice must be 0 or greater.");
    let bytes = s.as_bytes();
    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            if n <= 0 {
                return &s[0..i]
            } else {
                let j = i+1;
                return get_slice(&s[j..], n-1)
            }
        }
    }
    return &s[..]
}

pub fn demo() {

    let mut s = String::new();

    // create String from a string literal
    let s_literal = "start game";
    let mut s = s_literal.to_string(); // equivalent to String::from(s_literal)
    s.clear();
    s.push_str("Dobrý");
    s += " den";
    s.push('!');

    // all Rust strings contain UTF-8 encoded bytes
    let s_size = s.as_bytes().len();
    println!("String '{}' has byte length {}", s, s_size);

    {
        let s1 = String::from("foo");
        let s2 = String::from("bar");

        // with the '+' operator, the 1st argument is "moved" into the result string
        let s3 = s1 + &s2;

        // format doesn't take ownership of any of it's parameters
        let s4 = format!("{}-{}-{}", s2, &s3, "baz");

        //println!("s1: {}" , s1); illegal!
        println!("s2 : {}", s2);
        println!("s3 : {}", s3);
        println!("s4 : {}", s4);
    }

    {
        // get() and [i] are not legal operations on a Rust String
        // due the variable-length nature of UTF-8 chars (1 or 2 bytes)
        // Rust can't guarantee O(1) time complexity for the index operation
        let s = String::from("hдmu and egg");
        /*
        let ch0 = s[0];
        let ch1 = s.get(1);
        */

        // you can however slice a string using indexes, but this is dangerous...
        let str1 = &s[0..4]; // ok
        println!("ok slice = {}", str1);
        // Rust will PANIC at runtime if you slice a 2-byte UTF character in half!
        //let str2 = &s[0..2];
        //println!("bad slice = {}", str2);

        // you cannot slice .chars() or .bytes()
    }

}
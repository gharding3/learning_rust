// unlike Vector and String, HashMap is not part of the default prelude
use std::collections::HashMap;
use std::collections::hash_map::Entry;

/*
--- Notes about Rust hashing ---
Rust allows you to choose between different hashing implementations. These are called "hashers".
The default hasher is an implementation of SipHash, which is 
This was invented in 2012 to combat "hash flooding" denial-of-service attacks.
This algorithm uses a secret key to make it harder to find messages that hash to the same key.

Hash flooding occurs when an attacker sends a large number of messages that hash to the same key,
and then perform lookups of those messages.
Hash tables will react to a large number of collisions with "linear probing" for an empty slot.
This increases the lookup (and insert) time complexity from O(1) to O(n) where n is the number of collisions generated.
 */

pub fn demo() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Oilers"), 4);
    scores.insert(String::from("Kings"), 0);

    // inserting strings into a hashmap "moves" them
    let team_name = String::from("Canucks");
    scores.insert(team_name, 3);

    println!("map = {:?}", scores);

    // can zip together keys and values to form a map
    let teams = vec![String::from("Flames"), String::from("Ducks")];
    let goals = vec![2, 1];
    let mut scores: HashMap<String, i32> = teams.into_iter().zip(goals.into_iter()).collect(); // Rust knows we want a HashMap because of : HashMap<String,i32> specified above
    println!("map = {:?}", scores);

    // iterate through a map
    // order is arbitrary
    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }

    // get from a map
    if let Some(score) = scores.get("Flames") {
        println!("Flames scored {}", score);
    } else {
        println!("Not found.");
    }

    match scores.get("Oilers") { 
        Some(score) => println!("Oilers scored {}", score),
        None => println!("Not found."),
    }

    // overwriting a value
    // ironically, "insert" will overwrite a value
    scores.insert(String::from("Ducks"), 7);

    // insert returns the previous value
    let previous = scores.insert(String::from("Flames"), 4);
    match previous {
        Some(old_score) => println!("Old Flames score was {}", old_score),
        _ => (),
    }

    // entry() returns an Entry enum, which represents a map entry that may or may not exist
    let current : Entry<String,i32> = scores.entry(String::from("Coyotes"));
    match current {
        std::collections::hash_map::Entry::Occupied(_) => println!("Entry exists."),
        std::collections::hash_map::Entry::Vacant(_) => println!("Entry does not exist."),
    }
    // use or_insert() to insert a map entry if it doesn't yet exist.
    let score_ref : &mut i32 = current.or_insert(2);
    *score_ref = -1;

    println!("map = {:?}", scores);

    let wc = word_count("Buy one get one free");
    println!("---Word count---\n{:?}", wc);
}

fn word_count(text : &str) -> HashMap<String, i32> {
    let mut words = HashMap::new();
    for token in text.split_whitespace() {
        let count = words.entry(String::from(token)).or_insert(0);
        *count += 1;
    }
    words
}
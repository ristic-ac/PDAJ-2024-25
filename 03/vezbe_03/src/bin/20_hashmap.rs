use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("Alice"), 50); // String is moved.
    map.insert(String::from("Bob"), 30);

    // Borrowing key-value pairs
    for (key, value) in &map {
        println!("{}: {}", key, value); // Borrowing the map immutably.
    }

    // Removing an entry
    let bob_score = map.remove("Bob"); // Entry is consumed (moved out).
    println!("Removed Bob's score: {:?}", bob_score);

    // Not consuming the map
    let keys_iter = map.keys();
    let keys: Vec<&String> = keys_iter.collect();
    println!("Keys: {:?}", keys);

    // Consuming the map
    let keys_iter = map.into_keys();
    let keys: Vec<String> = keys_iter.collect();
    println!("Keys: {:?}", keys);
}

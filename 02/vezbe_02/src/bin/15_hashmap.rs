use std::collections::HashMap;

fn main() {
    // Creating a new empty HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Inserting key-value pairs into the HashMap
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    scores.insert(String::from("Charlie"), 30);
    println!("Initial scores: {:?}", scores);

    // Accessing a value by key
    let alice_score = scores.get("Alice");
    match alice_score {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("No score found for Alice"),
    }

    // Iterating over the HashMap
    for (key, value) in &scores {
        println!("Player: {}, Score: {}", key, value);
    }

    // Updating the value of an existing key
    scores.insert(String::from("Alice"), 50); // Alice's score is updated
    println!("Scores after updating Alice: {:?}", scores);

    // Using entry() to conditionally insert a value if the key is not present
    scores.entry(String::from("David")).or_insert(40);
    println!("Scores after adding David: {:?}", scores);

    // Using entry() to modify the value if the key exists
    let alice_entry = scores.entry(String::from("Alice")).or_insert(0);
    *alice_entry += 10; // Adds 10 to Alice's score
    println!("Scores after modifying Alice using entry(): {:?}", scores);

    // Checking if a key exists
    if scores.contains_key("Bob") {
        println!("Bob is in the scores map");
    }

    // Upserting a value (insert if absent, update if present)
    scores.insert(String::from("Bob"), 25); // Bob's score is updated

    // Removing a key-value pair
    scores.remove("Charlie");
    println!("Scores after removing Charlie: {:?}", scores);

    // Clearing the HashMap (removes all entries)
    scores.clear();
    println!("Scores after clearing all entries: {:?}", scores);
}

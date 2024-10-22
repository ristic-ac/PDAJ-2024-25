fn main() {
    // A vector of tuples, where each tuple contains a String and an i32
    let mut people = vec![
        (String::from("Alice"), 30),
        (String::from("Bob"), 25),
        (String::from("Charlie"), 35),
    ];

    // Example 1: Consuming the collection (moving ownership)
    consume_tuples(people.clone());

    // Example 2: Borrowing the tuples immutably (read-only)
    borrow_immutable_tuples(&people);

    // Example 3: Borrowing the tuples mutably (modifying in place)
    borrow_mutable_tuples(&mut people);

    // Verify the changes after mutable borrow
    println!("Modified people vector:");
    for (name, age) in &people {
        println!("Name: {}, Age: {}", name, age);
    }
}

// Consuming the collection by taking ownership of each tuple
fn consume_tuples(people: Vec<(String, i32)>) {
    println!("\nConsuming the collection:");
    for (name, age) in people {
        println!("Name: {}, Age: {}", name, age); // Ownership of name and age is moved here
    }
    // The vector `people` is now consumed and cannot be used further
}

// Borrowing the collection immutably (read-only)
fn borrow_immutable_tuples(people: &Vec<(String, i32)>) {
    println!("\nBorrowing immutably:");
    for (name, age) in people.iter() {
        println!("Name: {}, Age: {}", name, age); // Borrowed, so we can still use the original vector
    }
    // The original `people` vector can still be used after this function because we only borrowed it
}

// Borrowing the collection mutably to modify the values
fn borrow_mutable_tuples(people: &mut Vec<(String, i32)>) {
    println!("\nBorrowing mutably (modifying in place):");
    for (name, age) in people.iter_mut() {
        *age += 1; // Increment age by 1 for each person
        println!("Modified Name: {}, New Age: {}", name, age);
    }
    // The original `people` vector is modified because we borrowed it mutably
}

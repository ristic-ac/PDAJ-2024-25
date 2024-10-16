fn main() {
    // Creating a new empty vector
    let mut numbers: Vec<i32> = Vec::new();

    // Adding elements to the vector using push
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("Vector after pushing elements: {:?}", numbers);

    // Accessing elements of the vector by index
    println!("First element: {}", numbers[0]);

    // Safely accessing elements using get (returns an Option)
    match numbers.get(3) {
        Some(value) => println!("Fourth element: {}", value),
        None => println!("No fourth element"),
    }

    // Iterating over the vector using a for loop
    for number in &numbers {
        println!("Iterating over vector, number: {}", number);
    }

    // Iterating with mutable references to modify the vector in place
    for number in &mut numbers {
        *number += 5; // Adding 5 to each element
    }
    println!("Vector after modifying each element: {:?}", numbers);

    // Removing the last element of the vector using pop
    match numbers.pop() {
        Some(value) => println!("Popped value: {}", value),
        None => println!("No value to pop"),
    }
    println!("Vector after popping an element: {:?}", numbers);

    // Inserting an element at a specific position
    numbers.insert(1, 25); // Inserting 25 at index 1
    println!("Vector after inserting 25 at index 1: {:?}", numbers);

    // Removing an element at a specific index
    let removed_element = numbers.remove(1); // Removes the element at index 1
    println!("Removed element: {}", removed_element);
    println!("Vector after removing element at index 1: {:?}", numbers);

    // Extending the vector by adding another vector
    let more_numbers = vec![40, 50, 60];
    numbers.extend(more_numbers);
    println!("Vector after extending with more numbers: {:?}", numbers);

    // Iterating with index using enumerate
    for (index, number) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, number);
    }

    // Clearing the vector (removes all elements)
    numbers.clear();
    println!("Vector after clearing all elements: {:?}", numbers);
}

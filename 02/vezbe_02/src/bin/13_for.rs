fn main() {
    // For loop over a range
    for i in 0..5 {
        println!("For loop over range, i: {}", i);
    }

    // For loop over an inclusive range
    for i in 0..=5 {
        println!("For loop over inclusive range, i: {}", i);
    }

    // For loop iterating over an array
    let arr = [10, 20, 30, 40, 50];
    for element in arr.into_iter() {
        println!("For loop over array, element: {}", element);
    }
    for element in arr.iter() {
        println!("For loop over array, element: {}", element);
    }

    // For loop iterating over a vector
    let vec = vec![1, 2, 3, 4, 5];
    for element in vec.iter() {
        println!("For loop over vector, element: {}", element);
    }
    for element in vec.iter() {
        println!("For loop over vector, element: {}", element);
    }

    // For loop with index using enumerate
    for (index, element) in arr.iter().enumerate() {
        println!(
            "For loop with enumerate, index: {}, element: {}",
            index, element
        );
    }

    // Nested for loops
    for x in 1..4 {
        for y in 1..4 {
            println!("Nested for loop, x: {}, y: {}", x, y);
        }
    }

    // For loop with break
    for i in 0..10 {
        if i == 5 {
            println!("Breaking the loop at i: {}", i);
            break; // Exit the loop when i is 5
        }
        println!("For loop, i: {}", i);
    }

    // For loop with continue
    for i in 0..5 {
        if i == 3 {
            println!("Skipping i: {}", i);
            continue; // Skip the rest of the current iteration
        }
        println!("For loop with continue, i: {}", i);
    }
}

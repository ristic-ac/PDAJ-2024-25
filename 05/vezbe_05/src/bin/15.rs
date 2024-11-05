fn main() {
    // Example 1: Explicit panic with a custom message
    panic!("Something went terribly wrong!");

    // Example 2: Accessing an invalid index in a vector (implicit panic)
    let numbers = vec![1, 2, 3];
    println!("The number at index 5 is: {}", numbers[5]); // This line will cause a panic!
}

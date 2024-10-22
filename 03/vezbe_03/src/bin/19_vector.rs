fn main() {
    let mut vec = vec![10, 20, 30];

    // Immutable borrow
    for &item in &vec {
        println!("Item: {}", item); // Borrowed, ownership not moved.
    }

    // Mutable borrow
    for item in &mut vec {
        *item += 10; // Modify the elements in place.
    }
    println!("Updated vector: {:?}", vec);

    // Consuming the vector
    let vec_iter = vec.into_iter();
    let sum: i32 = vec_iter.sum(); // vec is consumed and cannot be used after this.
    println!("Sum of elements: {}", sum);
}

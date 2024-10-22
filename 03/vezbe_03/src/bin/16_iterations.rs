fn main() {
    let vec = vec![1, 2, 3, 4];

    // Iterating over the vector immutably
    for elem in &vec {
        println!("{}", elem);
    }

    let mut vec = vec![1, 2, 3, 4];

    // Iterating over the vector mutably
    for elem in &mut vec {
        *elem += 1;
    }

    let vec = vec![1, 2, 3, 4];

    // Consuming the vector by taking ownership of each element
    for elem in vec {
        println!("{}", elem);
    }
    // `vec` can no longer be used here since its ownership has been consumed
}

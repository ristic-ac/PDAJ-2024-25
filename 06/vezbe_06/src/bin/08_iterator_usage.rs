fn main() {
    // Initialize a vector
    let v = vec![1, 2, 3];

    // Using `iter` to iterate over immutable references
    println!("Iterating with `iter`: ");
    for &x in v.iter() {
        println!("{}", x);  // Prints values: 1, 2, 3
    }

    // Using `into_iter` to take ownership of the elements
    println!("\nIterating with `into_iter`: ");
    let v2 = vec![4, 5, 6];
    let mut v2_iter = v2.into_iter();  // 'v2' now takes ownership of the elements
    println!("{}", v2_iter.next().unwrap());  // Prints: 4
    println!("{}", v2_iter.next().unwrap());  // Prints: 5
    println!("{}", v2_iter.next().unwrap());  // Prints: 6

    // Using `iter_mut` to iterate and modify with mutable references
    let mut v3 = vec![7, 8, 9];
    println!("\nIterating with `iter_mut` and modifying values: ");
    for x in v3.iter_mut() {
        *x += 10;  // Adds 10 to each element
    }
    for &x in v3.iter() {
        println!("{}", x);  // Prints: 17, 18, 19
    }
}

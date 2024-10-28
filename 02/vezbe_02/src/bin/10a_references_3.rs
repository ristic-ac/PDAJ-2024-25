fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    // We borrow the reference to the third element of the vector.
    let num: &mut i32 = &mut v[2];
    // We dereference the reference to the third element of the vector, and we create immutable reference to it.
    let num2: &i32 = &*num;
    // This in turn means that the num reference to the third element of the vector is immutable.
    // *num += 1;
    println!("{} {}", *num, *num2);
}
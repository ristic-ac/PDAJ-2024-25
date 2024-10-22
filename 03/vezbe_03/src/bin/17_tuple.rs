fn take_tuple_ints(t: (i32, i32)) {
    println!("Tuple contains: {}, {}", t.0, t.1);
}

fn take_tuple_int_string(t: (i32, String)) {
    println!("Tuple contains: {}, {}", t.0, t.1);
}

fn main() {
    let tup = (42, 99);
    take_tuple_ints(tup); // `tup` is copied, not moved, since both `i32` values implement `Copy`.
    println!("{}", tup.1); // This works because `tup` wasn't moved.

    let tup = (42, String::from("hello")); // In Rust, tuples will only implement Copy if all of their elements implement Copy. Since String does not implement Copy, the tuple (String, i32) as a whole does not implement Copy
    take_tuple_int_string(tup); // Ownership of the entire tuple (and thus the String inside it) is moved to the function.
    println!("{}", tup.1); // This doesnt work because `tup` was moved.
}

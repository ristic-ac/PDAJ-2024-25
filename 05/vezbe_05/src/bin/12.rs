fn main() {
    // Example 1: String literal with a 'static lifetime
    let s1: &'static str = "I have a static lifetime.";
    println!("{}", s1); // This string lives for the entire duration of the program.

    // Example 2: String allocated at runtime, which does NOT have a 'static lifetime
    let s2 = String::from("I have a shorter lifetime.");
    let s2_ref: &str = &s2; // `s2_ref` borrows from `s2`, so it doesn't have a 'static lifetime
    println!("{}", s2_ref);

    // Example 3: Trying to assign a non-static reference to a 'static variable - Will not compile
    // Uncommenting the following code will cause a compilation error
    // let s3: &'static str = &s2;
    // println!("{}", s3);
    // Error: `s3` would require `s2` to live for the entire program duration, which is not possible.

    // Example 4: Function that accepts a 'static string slice
    print_static_string("This is a static string literal.");
    
    // Example 5: Attempting to force a non-static string into a 'static context
    // This commonly occurs when functions require a 'static lifetime
    // Uncommenting the following code will cause a compilation error
    // let s4 = String::from("Temporary string");
    // print_static_string(&s4); // Error: `&s4` does not have a 'static lifetime
}

// A function that accepts only 'static string slices
fn print_static_string(s: &'static str) {
    println!("Static string: {}", s);
}

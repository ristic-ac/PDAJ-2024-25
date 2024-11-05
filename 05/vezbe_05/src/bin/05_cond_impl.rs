use std::fmt::Display;

// Define a generic struct Pair<T>
struct Pair<T> {
    x: T,
    y: T,
}

// Always implement the new function for Pair<T>
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Conditionally implement cmp_display if T: Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Example of a blanket implementation in the standard library
// impl<T: Display> ToString for T {
// // No need to implement ToString for T as it is already implemented for types that implement Display
// }

// Main function demonstrating conditional and blanket implementations
fn main() {
    let pair = Pair::new(3, 5);
    pair.cmp_display(); // Works because i32 implements Display and PartialOrd

    let float_pair = Pair::new(4.2, 1.8);
    float_pair.cmp_display(); // Works because f64 implements Display and PartialOrd

    // Blanket implementation example
    let num_str = 3.to_string(); // Works because i32 implements Display
    println!("Converted integer to string: {}", num_str);
}

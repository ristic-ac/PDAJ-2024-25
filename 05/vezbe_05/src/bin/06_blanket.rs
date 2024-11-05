use std::fmt::Display;

// Define a custom trait called Printable
trait Printable {
    fn print(&self);
}

// Provide a blanket implementation of Printable for any type that implements Display
impl<T: Display> Printable for T {
    fn print(&self) {
        println!("{}", self);
    }
}

// Test struct without implementing Printable directly
struct User {
    name: String,
}

// Implement Display for User so it can use Printable automatically
impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User: {}", self.name)
    }
}

fn main() {
    // Using Printable for types with Display
    let number = 42;
    let user = User {
        name: String::from("Alice"),
    };

    // Calling the print method from Printable, which works because of the blanket implementation
    number.print(); // Works because i32 implements Display
    user.print();   // Works because User implements Display
}

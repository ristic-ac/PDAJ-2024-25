// Automatically implement the Debug trait for AlwaysEqual, allowing it to be printed with {:?}
#[derive(Debug)]
struct AlwaysEqual;

// Implement the PartialEq trait for AlwaysEqual, allowing comparison with `==`
// Purpose of PartialEq: The PartialEq trait allows us to define custom equality logic for types, 
// meaning we can decide what it means for two values to be "equal" (==).
// Eq is automatically added to any type with a consistent PartialEq implementation.
impl PartialEq for AlwaysEqual {
    // Define the `eq` method to always return true, meaning any instance of AlwaysEqual
    // will always be considered equal to any other instance of AlwaysEqual
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

fn main() {
    let subject1 = AlwaysEqual; // Create an instance of AlwaysEqual
    let subject2 = AlwaysEqual; // Create another instance of AlwaysEqual

    // Use the `==` operator, which calls the `eq` method, which always returns true
    if subject1 == subject2 {
        // Since `eq` always returns true, this branch will always execute
        println!("subject1 is always equal to subject2");
    } else {
        // This branch will never execute because `eq` is hardcoded to return true
        println!("This will never print");
    }

    // Use the function to compare the two instances
    if subject1.eq(&subject2) {
        // This branch will always execute because `eq` is hardcoded to return true
        println!("subject1 is always equal to subject2");
    } else {
        // This branch will never execute because `eq` is hardcoded to return true
        println!("This will never print");
    }

    if subject1 != subject2 {
        // This branch will never execute because `ne` is hardcoded to return false
        println!("This will never print");
    } else {
        // Since `ne` always returns false, this branch will always execute
        println!("subject1 is always equal to subject2");
    }
}

// Automatically implement the Debug trait for AlwaysEqual, allowing it to be printed with {:?}
#[derive(Debug, Eq)]
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

// Eq has no methods to implement, but you must implement PartialEq to use Eq
// Also, for Eq to be correct, Reflexivity, Symmetry, and Transitivity of eq must be satisfied
// Eq just makes guarantees about the RST behavior of PartialEq, allowing use of HashSet and HashMap

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflexivity() {
        let a = AlwaysEqual;
        assert_eq!(a, a); // Reflexivity: a == a should be true
    }

    #[test]
    fn test_symmetry() {
        let a = AlwaysEqual;
        let b = AlwaysEqual;
        assert_eq!(a, b); // Symmetry: a == b
        assert_eq!(b, a); // b == a should also be true
    }

    #[test]
    fn test_transitivity() {
        let a = AlwaysEqual;
        let b = AlwaysEqual;
        let c = AlwaysEqual;
        assert_eq!(a, b); // a == b
        assert_eq!(b, c); // b == c
        assert_eq!(a, c); // a == c (transitivity)
    }
}
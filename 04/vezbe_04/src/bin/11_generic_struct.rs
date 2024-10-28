// Structure with generic type for x and y
// Both x and y must be of the same type
// The type of x and y is defined when the structure is created
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Implementation of the structure
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}", integer);
    println!("{:?}", float);

    println!("x of integer: {}", integer.x());
    println!("x of float: {}", float.x());
}
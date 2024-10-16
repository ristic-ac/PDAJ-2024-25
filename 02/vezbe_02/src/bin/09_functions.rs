// Basic function declaration with no return value (unit type `()` is implied)
fn say_hello() {
    println!("Hello, world!");
}

// Function with parameters and an explicit return type (i32)
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function with an early return using `return` keyword (optional, as Rust returns the last expression by default)
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    false
}

// Recursive function: calculates factorial of a number
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Function with default parameters using optional arguments
fn greet(name: Option<&str>) {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => println!("Hello, stranger!"),
    }
}

fn main() {
    // Call the basic function
    say_hello();

    // Call the add function
    let sum = add(5, 7);
    println!("Sum of 5 and 7: {}", sum);

    // Check if a number is even
    let number = 10;
    println!("Is {} even? {}", number, is_even(number));

    // Call the recursive factorial function
    let fact_of_5 = factorial(5);
    println!("Factorial of 5: {}", fact_of_5);

    // Call function with optional arguments
    greet(Some("Alice"));
    greet(None);
}

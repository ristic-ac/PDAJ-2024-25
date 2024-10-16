fn main() {
    let x = 10;
    let y = 5;

    // Basic if condition
    if x > y {
        println!("x is greater than y");
    }

    // if-else condition
    if x == y {
        println!("x is equal to y");
    } else {
        println!("x is not equal to y");
    }

    // Nested if-else
    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
    }

    // if as an expression
    // Assigns a value based on the condition
    let max = if x > y { x } else { y };
    println!("The maximum value is: {}", max);

    // Using if with multiple conditions (logical AND and OR)
    if x > 0 && y > 0 {
        println!("Both x and y are positive numbers");
    }

    if x < 0 || y < 0 {
        println!("At least one of x or y is negative");
    } else {
        println!("Neither x nor y are negative");
    }

    // Handling more complex conditions
    let z = 3;

    if x > y && y > z {
        println!("x is the largest and z is the smallest");
    } else if z > x {
        println!("z is the largest");
    } else {
        println!("Neither condition is true");
    }

    // Using if-let to work with Option types
    let some_value: Option<i32> = Some(100);

    if let Some(v) = some_value {
        println!("Found a value: {}", v);
    } else {
        println!("No value found");
    }
}

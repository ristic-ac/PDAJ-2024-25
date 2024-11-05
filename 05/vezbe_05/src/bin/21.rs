fn main() {
    // Example inputs
    let input_some = Some("42");
    let input_none: Option<&str> = None;
    let input_invalid = Some("not_a_number");

    // Run the examples
    println!("Processing Some(\"42\"):");
    let result_some = process_number(input_some);
    println!("Result: {:?}", result_some); // Should return Some(84)

    println!("Processing None:");
    let result_none = process_number(input_none);
    println!("Result: {:?}", result_none); // Should return None

    println!("Processing Some(\"not_a_number\"):");
    let result_invalid = process_number(input_invalid);
    println!("Result: {:?}", result_invalid); // Should return None
}

fn process_number(input: Option<&str>) -> Option<i32> {
    // Step 1: Unwrap the input Option<&str> with ?
    let num_str = input?;
    println!("Unwrapped number: {:?}", num_str);
    
    // Step 2: Try parsing the string as an integer with ?
    let num = num_str.parse::<i32>().ok()?;
    println!("Parsed number: {:?}", num);

    // Step 3: Perform a calculation (e.g., double the number)
    let doubled = num * 2;
    println!("Doubled number: {:?}", doubled);
    Some(doubled)
}

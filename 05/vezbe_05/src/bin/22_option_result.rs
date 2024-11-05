fn main() {
    // 1. Convert Result to Option using .ok()
    let result_to_option: Result<i32, &str> = Ok(42);
    let option_from_result = result_to_option.ok(); // Converts Ok(42) to Some(42)

    match option_from_result {
        Some(value) => println!("Converted Result to Option: Some({})", value),
        None => println!("Converted Result to Option: None"),
    }

    // Example with Err
    let error_result: Result<i32, &str> = Err("Error occurred");
    let none_from_error = error_result.ok(); // Converts Err to None

    match none_from_error {
        Some(value) => println!("Converted Result to Option: Some({})", value),
        None => println!("Converted Result to Option: None (error was ignored)"),
    }

    // 2. Convert Option to Result using .ok_or()
    let option_to_result: Option<i32> = Some(100);
    let result_from_option = option_to_result.ok_or("No value found"); // Converts Some(100) to Ok(100)

    match result_from_option {
        Ok(value) => println!("Converted Option to Result: Ok({})", value),
        Err(err) => println!("Converted Option to Result: Err({})", err),
    }

    // Example with None
    let none_option: Option<i32> = None;
    let err_from_none = none_option.ok_or("No value present"); // Converts None to Err("No value present")

    match err_from_none {
        Ok(value) => println!("Converted Option to Result: Ok({})", value),
        Err(err) => println!("Converted Option to Result: Err({})", err),
    }
}

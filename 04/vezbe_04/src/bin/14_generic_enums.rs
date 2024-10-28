// Vec implements the Deref trait to &[T], which enables this implicit conversion. 
// When you call find_word(words: &[&str], target: &str), Rust sees that find_word expects a &[&str], 
// so it automatically dereferences &Vec<&str> to a slice (&[&str]).
fn find_word(words: &[&str], target: &str) -> Option<usize> {
    // Loop through the words to find the target
    for (index, &word) in words.iter().enumerate() {
        if word == target {
            return Some(index);
        }
    }
    None
}

fn divide_numbers(dividend: f64, divisor: f64) -> Result<f64, String> {
    // Check if division is possible (divisor not zero)
    if divisor == 0.0 {
        Err("Cannot divide by zero.".to_string())
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    // Using Option to handle optional values
    let words = vec!["apple", "banana", "cherry"];
    let target = "banana";
    let borrowed_words = &words;
    let result = find_word(&words, target);
    match result {
        Some(index) => println!("Found '{}' at index: {}", target, index),
        None => println!("'{}' not found", target),
    }

    let missing_word = "grape";
    let result = find_word(&words, missing_word);
    match result {
        Some(index) => println!("Found '{}' at index: {}", missing_word, index),
        None => println!("'{}' not found", missing_word),
    }

    // Using Result to handle potential errors
    let result = divide_numbers(10.0, 2.0);
    match result {
        Ok(result) => println!("10.0 divided by 2.0 is: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    let result = divide_numbers(10.0, 0.0);
    match result {
        Ok(result) => println!("10.0 divided by 0.0 is: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

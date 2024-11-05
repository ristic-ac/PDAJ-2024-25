// src/main.rs
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("This is a long string");

    {
        let string2 = String::from("Short");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
}

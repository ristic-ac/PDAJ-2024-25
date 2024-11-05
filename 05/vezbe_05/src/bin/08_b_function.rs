// src/main.rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
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

    // Uncommenting the lines below will cause a compile error
    // because `string2` is no longer in scope.
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is '{}'", result); // This will cause an error
}

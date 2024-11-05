use std::fmt::Display;

// Define a struct that holds a reference, requiring a lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // A method that returns the part field and enforces the lifetime
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Function that returns the longer of two string slices, with a lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Function that returns the longer of two string slices, with lifetime annotations
fn longest_outlive<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a, // 'b outlives 'a, which means reference y must outlive reference x
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Function that attempts to return a reference to a local variable (won't compile)
// Uncommenting this code will cause a compile-time error
// fn invalid_reference<'a>() -> &'a str {
//     let local_string = String::from("I'm temporary!");
//     &local_string // Error: `local_string` does not live long enough
// }

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    // Create an instance of ImportantExcerpt, holding a reference to part of `novel`
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    // Using a method that returns a reference, with lifetime enforced by struct's lifetime
    println!("Excerpt part: {}", excerpt.announce_and_return_part("Here's an important excerpt"));

    // Example of longest function with lifetime annotations
    let string1 = String::from("Hello");
    let string2 = "world!";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    // Example of longest function with lifetime annotations and outliving references
    let string3 = String::from("This is a long string");
    let string4 = "Short";
    let result = longest_outlive(string3.as_str(), string4);
    println!("The longest string is: {}", result);

}

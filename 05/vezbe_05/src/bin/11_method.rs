// Define a struct with a lifetime parameter `'a`
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Method with no lifetimes in the parameters or return type, so no explicit lifetime annotations needed
    fn level(&self) -> i32 {
        3
    }

    // Method that takes a reference and returns a reference to a part of the struct.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("In a galaxy far, far away...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // `excerpt` holds a reference to `first_sentence`, which has a shorter lifetime
    let excerpt = ImportantExcerpt { part: first_sentence };

    // Using the `level` method
    println!("Level: {}", excerpt.level());

    // Using `announce_and_return_part` with a string literal as an announcement
    let announcement = "Breaking news!";
    let part = excerpt.announce_and_return_part(announcement);
    println!("Part of the excerpt: {}", part);
}

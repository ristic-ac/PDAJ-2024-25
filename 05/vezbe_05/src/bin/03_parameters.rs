use std::fmt::{Debug, Display};

// Define a Summary trait with a default implementation
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implement Summary trait for NewsArticle
#[derive(Clone, Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

// Implement Summary and Display traits for Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "@{}: {}", self.username, self.content)
    }
}

// Function using impl Trait for single parameter
pub fn notify_simple(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Function with trait bound syntax, supporting multiple parameters of same type
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Function with multiple parameters, allowing different types using impl Trait
pub fn notify_multiple(item1: &impl Summary, item2: &impl Summary) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

// Function with multiple parameters, enforcing the same type using trait bound
pub fn notify_same_type<T: Summary>(item1: &T, item2: &T) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

// Function requiring multiple trait bounds using + syntax
pub fn notify_with_display(item: &(impl Summary + Display)) {
    println!("Notified: {}", item);
}

// Function with multiple bounds using where clause for readability
pub fn multiple_bounds_where<T, U>(t: &T, u: &U)
where
    T: Summary + Display,
    U: Clone + Debug,
{
    println!("T: {}, U: {:?}", t.summarize(), u);
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    // Demonstrate various function calls
    notify_simple(&article);            // uses impl Trait
    notify(&tweet);                     // uses trait bound syntax
    notify_multiple(&article, &tweet);  // different types using impl Trait
    notify_same_type(&tweet, &tweet);   // same type enforced with trait bound
    notify_with_display(&tweet);        // multiple trait bounds using + syntax

    // Example using where clause for multiple bounds
    multiple_bounds_where(&tweet, &article);
}

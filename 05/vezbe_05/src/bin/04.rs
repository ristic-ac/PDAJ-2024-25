// Define the Summary trait with a summarize method
pub trait Summary {
    fn summarize(&self) -> String;
}

// Implement Summary for Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// Implement Summary for NewsArticle
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} - by {}, {}", self.headline, self.author, self.location)
    }
}

// Function returning a single type implementing Summary (Tweet)
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Attempting to return multiple types - this does NOT compile
// fn returns_either(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Correct approach: returning different types using Box<dyn Summary>
fn returns_summarizable_trait_object(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        })
    } else {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        })
    }
}

fn main() {
    // Using the impl Trait function
    let tweet_summary = returns_summarizable();
    println!("Tweet summary: {}", tweet_summary.summarize());

    // Using the trait object function to handle different types
    let article_summary = returns_summarizable_trait_object(true);
    let tweet_summary2 = returns_summarizable_trait_object(false);
    println!("Article summary: {}", article_summary.summarize());
    println!("Another tweet summary: {}", tweet_summary2.summarize());
}

pub struct NewsArticle {
    pub author: String, 
    pub headline: String, 
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        // comment out to see the default call
        format!("{}: by {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

pub trait Summary {
    //must declare a method, but does not provide a default behavior
    //fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

// takes in something that implements summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

fn main() {
    let article = NewsArticle {
        author: String::from("me"),
        headline: String::from(
            "Traits yo",
        ),
        content: String::from("Say what"),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize())
}


use std::fmt::Display;
// putting it all together including lifetimes
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
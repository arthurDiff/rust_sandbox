use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read More...) from {}", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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
        format!("{}: {}", self.content, self.username)
    }

    fn summarize_author(&self) -> String {
        String::from(self.username.as_str())
    }
}

pub fn notify(item: &impl Summary) {
    println!("This is summary: {}", item.summarize())
}

pub fn notify_v2(item: &(impl Summary + Display)) {}

pub fn notify_v3<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Summary,
    U: Clone,
{
    8
}

pub fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("something"),
        content: String::from("Some content here"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x < self.y {
            println!("something y: {}", self.y)
        } else {
            println!("something x: {}", self.x)
        }
    }
}

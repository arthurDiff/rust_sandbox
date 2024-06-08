mod aggregator;
mod lifetime;
use aggregator::{notify, NewsArticle, Summary};
use lifetime::longest_string;
fn main() {
    println!("{}", largest(&[3, 5, 42, 3, 4, 2, 4, 5, 332]));
    println!("{:?}", Point { x: 23., y: 3234.3 });

    let news = NewsArticle {
        author: String::from("michael"),
        headline: String::from("This is headline"),
        location: String::from("Chicago"),
        content: String::from("placeholder texts"),
    };
    println!("{}", news.summarize());
    notify(&news);
    let f_str = "first string here ";
    let s_str = "second string here which is longer";
    println!("{}", longest_string(f_str, s_str))
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for val in list {
        if largest < val {
            largest = val
        }
    }
    largest
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

#[allow(dead_code)]
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

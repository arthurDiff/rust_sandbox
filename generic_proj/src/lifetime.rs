use std::fmt::Display;

pub fn longest_string<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    }
    str2
}

pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime Elision Rules
fn longest_with_annot<'a, T>(x: &'a str, y: &'a str, z: T) -> &'a str
where
    T: Display,
{
    println!("Announce {} {} {}", x, y, z);
    x
}

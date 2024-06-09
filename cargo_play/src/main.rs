//! # My Crate
//!
//! This is my crate

use cargo_play::something_here;
fn main() {
    println!("Hello, world!");
    something_here()
}

/// Adds eight to number taking ownership of passed var
///
/// # Example
///
/// ```
/// let arg = 3;
/// let answer = my_crate::add_eight(arg);
///
/// assert_eq!(answer, 11);
/// ```
fn add_eight(num: i32) -> i32 {
    num + 8
}

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let _lunch = module_sys::Lunch::summer("bread");
    let x = String::from("hello");
    let y = String::from("World");
    println!("{}", x + &y);
    let mut h_map = HashMap::new();
    h_map.insert("hello", "world");
    println!("{:?}", h_map.get("hello"));
    h_map.insert("hello", "somethingelse");
    println!("{:?}", h_map.get("hello"));
    h_map.entry("hello").or_insert("new value");
    println!("{:?}", h_map.get("hello"));
}

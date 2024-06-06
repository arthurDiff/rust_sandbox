fn main() {
    println!("Hello, world!");
    let _lunch = module_sys::Lunch::summer("bread");
    let x = String::from("hello");
    let y = String::from("World");
    println!("{}", x + &y);
}

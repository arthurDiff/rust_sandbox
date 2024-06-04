#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        dbg!(self.width * self.height)
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width < self.width && rect.height < self.height
    }
}
fn main() {
    let rect = Rectangle::new(35, 88);
    let rect2 = Rectangle::new(23, 43);
    println!("area of 35, 88 rectacle is {}, {:?}", rect.area(), rect);
    println!("can rect hold rect2? {}", rect.can_hold(&rect2));
}

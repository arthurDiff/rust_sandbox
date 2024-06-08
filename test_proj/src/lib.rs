#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        dbg!(self.width * self.height)
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width < self.width && rect.height < self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn larget_can_hold_small() {
        let larg = Rectangle::new(24, 25);
        let small = Rectangle::new(4, 3);
        assert!(larg.can_hold(&small))
    }

    #[test]
    fn small_cant_hold_large() {
        let larg = Rectangle::new(24, 25);
        let small = Rectangle::new(4, 3);
        assert!(!small.can_hold(&larg))
    }

    #[test]
    fn is_same_square_size() {
        let s1 = Rectangle::new(55, 225);
        let s2 = Rectangle::new(55, 225);
        assert_eq!(
            s1, s2,
            "Two Rectangle didn't match width or height => Rect1: {:?} | Rect2: {:?}",
            s1, s2
        )
    }

    #[test]
    #[should_panic(expected = "panics")]
    fn panic_test() {
        panic!("this panics");
    }

    #[test]
    fn this_using_result() -> Result<(), String> {
        if 3 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("This is error"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {}
}

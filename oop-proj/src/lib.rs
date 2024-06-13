pub use blog::*;
mod blog;
mod blog_r;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        // trait objects allow for multiple concrete types to fill in for the trait object at runtime.
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing Button with width: {}, height: {}",
            self.width, self.height
        );
    }
}

pub struct TextField {
    pub font_size: i32,
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!(
            "Drawing TextField with font: {}, placeholder: {}",
            self.font_size, self.placeholder
        );
    }
}

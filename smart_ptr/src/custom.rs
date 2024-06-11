use std::ops::{Deref, DerefMut};

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(val: T) -> MyBox<T> {
        MyBox(val)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox is dropped");
    }
}

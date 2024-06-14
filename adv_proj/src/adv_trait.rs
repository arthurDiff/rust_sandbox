use std::{fmt::Debug, ops::Add};

pub fn run() {
    let x = Rand { something: 3 };
    Something::something(&x);
    x.something();
    //<Type as Trait>::function(receiver_if_method, next_arg, ...);
    <Rand as Something>::else_f();
}

trait Something: Debug {
    type S;
    fn something(&self) {
        println!("'sdf'");
    }
    fn else_f() {
        println!("else");
    }
}
#[derive(Debug)]
struct Rand {
    something: i32,
}
impl Rand {
    pub fn something(&self) {
        println!("direcl");
    }
    pub fn else_f() {
        println!("rand else")
    }
}

impl Something for Rand {
    type S = i32;
}

struct Point {
    x: i32,
}
struct Edge {
    x: i32,
}
impl Add<Edge> for Point {
    type Output = Point;
    fn add(self, rhs: Edge) -> Self {
        Point { x: 32 }
    }
}

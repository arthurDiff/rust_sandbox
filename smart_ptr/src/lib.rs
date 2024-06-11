use custom::MyBox;

mod custom;
pub fn run() {
    let list = List::Cons(32, Box::new(List::Cons(34, Box::new(List::Nil))));
    println!("{:?}", list);

    let x = 8;
    let y = MyBox::new(x);
    assert_eq!(x, 8);
    assert_eq!(*y, 8);
    drop(y);
    let w = MyBox::new("Something here");
    let text = MyBox::new(String::from("World"));
    //Deref coercion
    hello(&text)
}

// cons(construct function) list
#[derive(Debug)]
enum List<T = i32> {
    Cons(T, Box<List>),
    Nil,
}

fn hello(text: &str) {
    println!("hello {}", text);
}

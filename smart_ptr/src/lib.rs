use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use custom::MyBox;

mod custom;
mod messanger;
pub fn run() {
    let list_box = ListBox::Cons(32, Box::new(ListBox::Cons(34, Box::new(ListBox::Nil))));
    println!("{:?}", list_box);

    let list_rc1 = Rc::new(ListRC::Cons(
        4,
        Rc::new(ListRC::Cons(43434, Rc::new(ListRC::Nil))),
    ));
    let list_rc2 = ListRC::Cons(3, Rc::clone(&list_rc1));
    let list_rc3 = ListRC::Cons(2, Rc::clone(&list_rc1));
    println!(
        "{:?} {:?} also count for og rc is: {}",
        list_rc2,
        list_rc3,
        Rc::strong_count(&list_rc1)
    );

    let test_rc = Rc::new(345235);
    let text_rrr = Rc::downgrade(&test_rc);
    match text_rrr.upgrade() {
        Some(v) => v,
        None => Rc::new(0),
    };
    let val_refcell = Rc::new(RefCell::new(4));

    let a = Rc::new(ListRefCell::Cons(
        Rc::clone(&val_refcell),
        Rc::new(ListRefCell::Nil),
    ));

    let b = ListRefCell::Cons(Rc::new(RefCell::new(93)), Rc::clone(&a));

    *val_refcell.borrow_mut() = 24242;

    println!("{:?}", b);

    let x = 8;
    let y = MyBox::new(x);
    assert_eq!(x, 8);
    assert_eq!(*y, 8);
    drop(y);
    let _w = MyBox::new("Something here");
    let text = MyBox::new(String::from("World"));
    //Deref coercion
    hello(&text)
}
fn hello(text: &str) {
    println!("hello {}", text);
}
// cons(construct function) list
#[derive(Debug)]
enum ListBox<T = i32> {
    Cons(T, Box<ListBox>),
    Nil,
}
//Reference Counter
#[derive(Debug)]
enum ListRC<T = i32> {
    Cons(T, Rc<ListRC>),
    Nil,
}

//Reference Counter
#[derive(Debug)]
enum ListRefCell<T = i32> {
    Cons(Rc<RefCell<T>>, Rc<ListRefCell>),
    Nil,
}

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

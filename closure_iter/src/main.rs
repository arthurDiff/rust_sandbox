use shirts::*;
use std::thread;

mod iterator;
mod shirts;

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Green,
            ShirtColor::Red,
            ShirtColor::Red,
            ShirtColor::Green,
        ],
    };

    let user_pref = Some(ShirtColor::Red);
    let giveaway = store.give_away(user_pref);

    println!("user one: {:?}", giveaway);

    let user_pref2 = None;
    let giveaway2 = store.give_away(user_pref2);

    println!("user one: {:?}", giveaway2);

    let mut list = vec![9, 3, 5, 212, 4];
    println!("list to pass to closure: {:?}", list);

    let mut borrows_mutable = || list.push(5);
    borrows_mutable();
    println!("after mut: {:?}", list);

    thread::spawn(move || println!("FROM THREAD: {:?}", list))
        .join()
        .unwrap();

    iterator::run();
}

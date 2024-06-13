use std::collections::btree_map::Keys;

struct Point {
    x: i32,
    y: i32,
}
enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
fn main() {
    let x = 1;
    match x {
        1 | 2 | 3 | 11 => "hi",
        25..=29 => "something here",
        8 => "eight",
        _ => "whateve",
    };

    let char = 'd';
    match char {
        'a'..='z' => "something",
        _ => "sdf",
    };
    let p = Point { x: 24, y: 24 };
    // let Point { x: a, y } = p;
    match p {
        Point { x: 4, y } => "something",
        Point { x, y: 25 } => "else",
        _ => "aok",
    };
    let msg = Message::ChangeColor(Color::RGB(1, 2, 2));
    match msg {
        Message::Quit => "sdfsdf",
        Message::ChangeColor(Color::RGB(x, y, z)) => "(x, y, z),",
        Message::ChangeColor(Color::HSV(x, y, z)) => "(x, y, z),",
        Message::Move { x, y } => "xy",
        Message::Write(str) => str.as_str(),
    };
    let ((x, y), (z, (a, b))) = ((1, 2), (3, (4, 5)));
    let p2 = Point { x: 24, y: 1221 };
    let Point { x, .. } = p2;

    let num = Some(6);
    match num {
        Some(x @ 1..=23) => 'a',
        Some(x) if x % 2 == 0 => 's',
        Some(x) => 'a',
        None => 'a',
    };
}

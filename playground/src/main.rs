#[allow(dead_code)]
fn main() {
    let mut test_val = Box::new(3);
    swap_to_8(&mut test_val);
    let _my_union = MyUnion { f1: 3 };
    println!("{}", test_val);
    println!("{}", clear_digits("cb34".into()));
}

pub fn clear_digits(s: String) -> String {
    let mut cs = vec![];
    let chars = s.chars();
    for c in chars {
        if c.is_ascii_digit() {
            if cs.is_empty() {
                continue;
            }
            _ = cs.pop()
        }
        cs.push(c)
    }
    cs.iter().collect()
}

fn swap_to_8(v: &mut Box<u8>) {
    let mut another = Box::new(8);

    std::mem::swap(&mut another, v);
}

#[allow(dead_code)]
union MyUnion {
    f1: u8,
    f2: u16,
    f3: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
enum MyError {
    First(std::io::Error),
    Seconf(std::io::Error),
    Unknown,
}

unsafe impl Send for MyError {}
unsafe impl Sync for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::First(_) => write!(f, "test"),
            MyError::Seconf(_) => write!(f, "test"),
            MyError::Unknown => write!(f, "test"),
        }
    }
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::First(error) => Some(error),
            MyError::Seconf(error) => Some(error),
            MyError::Unknown => None,
        }
    }
}

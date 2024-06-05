use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ColorChange(i32, i32, i32),
}

impl Message {
    #[allow(dead_code)]
    fn call(&self) -> &str {
        if let Self::Write(str_val) = self {
            return str_val.as_str();
        }
        ""
    }
}

fn main() {
    let home = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let office = IpAddr::V6(Ipv6Addr::new(9, 9, 9, 9, 9, 9, 9, 9));
    println!("{:?} {:?}", home, office);
}

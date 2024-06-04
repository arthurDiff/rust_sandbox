fn main() {
    let hello_world = String::from("Hello, world! Everyone");
    let hello = first_letter(&hello_world);
    let world = second_letter(&hello_world);
    println!("{}", hello);
    println!("{}", world);
}

fn first_letter(s: &str) -> &str {
    let byts = s.as_bytes();

    for (i, &item) in byts.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

fn second_letter(s: &String) -> &str {
    let byts = s.as_bytes();
    let mut first_space: Option<usize> = None;
    for (i, &item) in byts.iter().enumerate() {
        if item == b' ' {
            if let Some(n) = first_space {
                return &s[n + 1..i];
            }
            first_space = Some(i);
        }
    }
    ""
}

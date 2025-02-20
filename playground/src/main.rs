fn main() {
    println!("{}", clear_digits("abc".into()));
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

fn main() {
    println!("{:?}", make_smallest_palindrome("egcfe".to_owned()));
    println!("{:?}", make_smallest_palindrome("abcd".to_owned()));
    println!("{:?}", make_smallest_palindrome("seven".to_owned()))
}

pub fn make_smallest_palindrome(s: String) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    let char_len = chars.len();
    (0..=char_len / 2).for_each(|idx| {
        let (lhs, rhs) = (chars[idx], chars[char_len - 1 - idx]);
        if lhs == rhs {
            return;
        }
        if lhs > rhs {
            chars[idx] = chars[char_len - 1 - idx]
        } else {
            chars[char_len - 1 - idx] = chars[idx]
        }
    });
    chars.iter().collect()
}

use std::collections::HashMap;

fn main() {
    let (median, mode) = get_median_and_mode(vec![9, 2, 3, 2, 2, 2, 14, 253, 32, 332, 33]);
    println!("median: {}, mode: {}", median, mode);
    let cond_say = to_pig_latin(String::from("second"));
    let eight_hay = to_pig_latin(String::from("eight"));
    println!("{} {}", cond_say, eight_hay)
}

fn get_median_and_mode(nums: Vec<i32>) -> (i32, i32) {
    let num_length = nums.len();
    let mut num_map: HashMap<i32, i32> = HashMap::new();
    let mut medium: Option<i32> = None;
    let mut mode: (i32, i32) = (0, 0);
    for (i, num) in nums.iter().rev().enumerate() {
        if i == num_length / 2 {
            medium = Some(*num);
        }
        let num_val = num_map.entry(*num).and_modify(|c| *c += 1).or_insert(1);

        if mode.1 < *num_val {
            mode = (*num, *num_val)
        }
    }
    match medium {
        Some(n) => (n, mode.0),
        None => (0, mode.0),
    }
}

// Definitely not handling unicode correctly
fn to_pig_latin(str_val: String) -> String {
    let vowels = b"aeiou";
    if vowels.contains(&str_val[0..1].as_bytes()[0]) {
        return format!("{}-{}ay", &str_val[1..], &str_val[0..1]);
    }
    str_val + "-hay"
}

pub fn run() {
    let v1: Vec<i32> = vec![2, 45, 3, 3553, 124];
    let v1_iter = v1.iter();
    let new_iter: Vec<_> = v1_iter.map(|v| v * 2).filter(|v| v % 5 == 0).collect();
    for val in new_iter {
        println!("Got: {}", val);
    }
}

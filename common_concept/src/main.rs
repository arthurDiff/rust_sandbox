fn main() {
    returning_loop();
    println!(
        "This is 84 deg fahrenheight to celsius: {}",
        fahrenheith_to_celsius(84)
    );
    println!("This is 88th fibonacci number: {}", fibonacci(88));
}

fn returning_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 2;
        if counter > 12 {
            break counter * 8;
        }
    };
    println!("This is return value of loop: {}", result);
}

fn fahrenheith_to_celsius(deg: i32) -> i32 {
    (deg - 32) * 5 / 9
}

fn fibonacci(nth: usize) -> usize {
    let mut cache = vec![0, 1];
    if nth <= 1 {
        return nth;
    }
    for i in 2..=nth {
        cache.push(cache[i - 1] + cache[i - 2]);
    }
    cache[nth]
}

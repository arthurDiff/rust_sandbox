use std::io;

fn main() {
    println!("Intro");
    intro();
    println!("variables");
    variables();
    println!("data types");
    data_types();
    println!("console input");
    console_input();
    println!("arithmatic and casting");
    arithmatic_casting();
    println!("conditional");
    conditional();
    println!("functions");
    function_info();
    println!("memory management");
    memory_mgmt();
}

fn intro() {
    println!("hello world!");
}

fn variables() {
    // LET and Mutability
    let x = 8;
    println!("x is: {}", x);
    let mut mutable_x = 8;
    println!("mutableX is {}", mutable_x);
    mutable_x = 129;
    println!("mutated x is {}", mutable_x);
    // Shadowing
    {
        let x = 2323;
        println!("shadowed var {}", x);
    }

    let x = x - 2000;
    println!("override shadowed var {}", x);
    let x = "you can change variable upon redec";
    println!("{}", x);

    //CONST
    const CAPITAL_SNAKE_IS_CONVENTION: &str = "hello";
    println!("{}", CAPITAL_SNAKE_IS_CONVENTION);
}

fn data_types() {
    // SCALAR TYPES
    //  i8 i16 i32 i64 i128 -> same for u (unsigned) u8....
    let _i: i32 = 2;
    // f32 f64
    let _f: f64 = 8.8888;
    // bool
    let _b: bool = true;
    // char u8
    let _char: char = 'z';
    // COMPOUND TYPES
    let mut tup: (i32, bool, char) = (1, true, 'd');
    println!("{}", tup.1);
    tup.0 = 8;
    println!("{}", tup.0);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr[3]);
}

fn console_input() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    println!("{}", input);
}

fn arithmatic_casting() {
    let x: u8 = 18;
    let y: i8 = 19;
    let z = x + (y as u8);

    println!("{}", z);
    //casting to f32
    let _casted = 3.7_f32;
    let _casted = 3.7f32;
    let _casted = 3.7 as f32;
    //str conv
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("expected to readline");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 5);
}

fn conditional() {
    let _cond = 2 < 9; // 2 < 3.9 is invalid need types to match
                       // let _compound = !((2 < 9 && true) || false);

    let food = "kbbq";
    if food == "kbbq" {
        println!("nice");
    } else if food == "sushi" {
        println!("also good choice");
    } else {
        println!("no food for you");
    }
}

fn function_info() {
    fn add_numbers(x: i32, y: i32) -> i32 {
        // x + y without ; will return as well
        x + y
    }
    println!("{}", add_numbers(19, 3));
    let number = {
        let x = 34;
        x + 1
    };
    println!("{}", number);
}

fn memory_mgmt() {}

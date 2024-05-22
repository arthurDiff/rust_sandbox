fn main() {
    println!("Intro");
    intro();
    println!("variables");
    variables();
    println!("data types");
    data_types();
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

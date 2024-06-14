type Milimeter = i32;
type DynSendFn = Box<dyn Fn() + Send + 'static>;
enum Status {
    Value(u32),
    Stop,
}
pub fn run() {
    let x: Milimeter = 123;
    let test_fn: DynSendFn = Box::new(|| println!("asdasd"));
    println!("{}", do_twice(add_two, 1929));
    let x: Vec<u32> = vec![214, 24, 3];
    let z = (0u32..19);
    let y: Vec<Status> = (0u32..10).map(Status::Value).collect();
}
//Divergin Functions
fn never_returns() -> ! {
    loop {
        println!("TOLD YOU")
    }
}

fn generic<T: Sized>(t: T) {}
fn generic_nd<T: ?Sized>(t: &T) {}

fn add_two(x: i32) -> i32 {
    x + 2
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

fn return_closure() -> Box<dyn Fn(i32) -> String> {
    Box::new(|x| String::from("whu?"))
}

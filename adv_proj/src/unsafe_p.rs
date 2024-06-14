use core::slice;

static HELLO_WORD: &str = "HELLO WORLD";
static mut counter: i32 = 0;

extern "C" {
    fn abs(input: i32) -> i32;
}
pub fn run() {
    raw_poitner();
    unsafe {
        dangerous();
    }
}
fn raw_poitner() {
    let r1 = 394 as *const i32;
    let r2 = 3985 as *mut i32;

    let addr = 0x01234usize;
    let bad_raw_p = addr as *const usize;
    unsafe {
        println!("{}{}", *r1, *r2);
    }
}

unsafe fn dangerous() {
    println!("THIS IS UNSAFE FUNC");
}

fn split_at_mut<T>(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let ptr = values.as_mut_ptr();
    let len = values.len();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn extern_fns() {
    unsafe {
        println!("form c func{}", abs(-23));
    }
}

#[no_mangle]
pub extern "C" fn hi_from_rust() {
    println!("hi")
}

unsafe trait Foo {}

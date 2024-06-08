fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_something() {
        let result = 2 * 2;
        assert_eq!(result, 4);
    }
    // #[test]
    // fn failing_test() {
    //     panic!("This test should fail");
    // }
}

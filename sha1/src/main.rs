// https://www.thespatula.io/rust/rust_sha1/
fn main() {
    let mut sha1 = sha1::Sha1::new();
    println!("{:?}", sha1.hash("Test hash"));
}

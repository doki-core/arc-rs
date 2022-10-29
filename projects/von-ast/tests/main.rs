use std::path::PathBuf;

#[test]
fn ready() {
    println!("it works!")
}
#[test]
fn here() {
    println!("{:?}", PathBuf::from("./").canonicalize());
}

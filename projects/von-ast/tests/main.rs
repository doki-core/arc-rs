use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[test]
fn ready() {
    println!("it works!")
}
#[test]
fn here() {
    println!("{:?}", PathBuf::from("./").canonicalize());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestSerde {
    num1: u32,
}

#[test]
fn main() {
    let test = TestSerde { num1: 0 };
    println!("Orig {:?}", test);

    let serialized = serde_json::to_string(&test).expect("err ser");

    println!("Seri {}", serialized);

    let unse: TestSerde = serde_json::from_str(&serialized).expect("err unser");
    println!("New {:?}", unse);
}

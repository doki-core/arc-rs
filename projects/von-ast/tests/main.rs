use diagnostic::TextStorage;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use von::parse;

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

fn test_atom() {
    let mut store = TextStorage::default();
    let file1 = store.anonymous("1cm");
    let file2 = store.anonymous("0.1m");

    let text1 = &store.get(&file1).unwrap().source;
    let text2 = &store.get(&file2).unwrap().source;

    let ast1 = parse(&text1, &file1);
    let ast2 = parse(&text2, &file1);

    println!("{:#?}", ast1);
    println!("{:#?}", ast2);
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

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use voml_error::{TextStorage, Validation};
use von::{parse, VonNode};

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
fn test_atom() {
    let mut store = TextStorage::default();
    let ast1 = parse_from_store(&mut store, "1cm");
    let ast2 = parse_from_store(&mut store, "true");
    let ast3 = parse_from_store(&mut store, "()");
    println!("{:#?}", ast1);
    println!("{:#?}", ast2);
    println!("{:#?}", ast3);
}

#[track_caller]
fn parse_from_store(store: &mut TextStorage, input: &str) -> Validation<VonNode> {
    let file = store.anonymous(input);
    let text = &store.get(&file).unwrap().source;
    parse(&text, &file)
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

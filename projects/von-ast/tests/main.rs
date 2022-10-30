use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
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
    let ast1 = parse_text_store(&mut store, "1cm");
    let ast2 = parse_file_store(&mut store, "tests/basic.von");
    println!("{:#?}", ast1);
    println!("{:#?}", ast2);
}

#[track_caller]
fn parse_text_store(store: &mut TextStorage, input: &str) -> Validation<VonNode> {
    let file = store.anonymous(input);
    let text = store.get_text(&file).unwrap();
    parse(&text, &file)
}

#[track_caller]
fn parse_file_store(store: &mut TextStorage, input: impl AsRef<Path>) -> Validation<VonNode> {
    let file = store.file(input).unwrap();
    let text = store.get_text(&file).unwrap();
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

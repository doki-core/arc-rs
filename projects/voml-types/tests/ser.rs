use std::collections::BTreeMap;
use voml_types::{Deserializer, Von};

use crate::{TestStruct, TestStructUnit, TestTuple, TestTupleUnit, TestUnit};

#[test]
fn test_structure() {
    let der = Deserializer::default();
    assert_eq!(der.deserialize_object(&TestUnit).unwrap(), Von::list("TestUnit", vec![]));
    assert_eq!(der.deserialize_object(&TestTupleUnit()).unwrap(), Von::list("TestTupleUnit", vec![]));
    assert_eq!(der.deserialize_object(&TestStructUnit {}).unwrap(), Von::list("TestStructUnit", vec![]));
    assert_eq!(
        der.deserialize_object(&TestTuple(0, 1, 2)).unwrap(),
        Von::list("TestTuple", vec![Von::number("", 0), Von::number("", 1), Von::number("", 2),])
    );
    // assert_eq!(der.deserialize_object(&TestStruct { a: 0, b: 1, c: 2 }).unwrap(), Von::list("TestStruct", vec![]));
}

#[test]
fn test_map() {
    let der = Deserializer::default();
    let mut map = BTreeMap::new();
    map.insert(true, true);
    map.insert(false, false);

    assert_eq!(der.deserialize_object(&map).unwrap(), Von::list("TestUnit", vec![]));
}

#[test]
fn test_atom2() {
    // println!("{:#?}", TestEnum::Nothing.serialize(ObjectSerializer::default()).unwrap());
    // println!("{:#?}", TestEnum::Tuple(0, 0).serialize(ObjectSerializer::default()).unwrap());
    // println!("{:#?}", TestEnum::Struct { x: 0, y: 0 }.serialize(ObjectSerializer::default()).unwrap());
}

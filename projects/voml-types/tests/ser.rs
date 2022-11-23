use serde::Serialize;

use voml_types::{Serializer, Von};

use crate::{TestEnum, TestStruct, TestStructUnit, TestTuple, TestTupleUnit, TestUnit};

#[test]
fn test_atom() {
    let von = TestUnit.serialize(Serializer::default()).unwrap();
    assert_eq!(von, Von::list("TestUnit", vec![]));
    let von = TestTupleUnit().serialize(Serializer::default()).unwrap();
    assert_eq!(von, Von::list("TestTupleUnit", vec![]));
    let von = TestStructUnit {}.serialize(Serializer::default()).unwrap();
    assert_eq!(von, Von::list("TestStructUnit", vec![]));

    println!("{:#?}", TestTuple(0, 1, 2).serialize(Serializer::default()).unwrap());
    println!("{:#?}", TestStruct { a: 0, b: 1, c: 2 }.serialize(Serializer::default()).unwrap());
}

#[test]
fn test_atom2() {
    println!("{:#?}", TestEnum::Nothing.serialize(Serializer::default()).unwrap());
    println!("{:#?}", TestEnum::Tuple(0, 0).serialize(Serializer::default()).unwrap());
    println!("{:#?}", TestEnum::Struct { x: 0, y: 0 }.serialize(Serializer::default()).unwrap());
}

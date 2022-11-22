use serde::Serialize;

use voml_types::VonSerializer;

use crate::{TestStruct, TestStructUnit, TestTuple, TestTupleUnit, TestUnit};

#[test]
fn test_atom() {
    println!("{:#?}", TestUnit.serialize(VonSerializer {}).unwrap())
}

#[test]
fn test_atom2() {
    println!("{:#?}", TestTupleUnit().serialize(VonSerializer {}).unwrap());
    println!("{:#?}", TestTuple(0, 1, 2).serialize(VonSerializer {}).unwrap());
    println!("{:#?}", TestStruct { a: 0, b: 1, c: 2 }.serialize(VonSerializer {}).unwrap());
    println!("{:#?}", TestStructUnit {}.serialize(VonSerializer {}).unwrap());
}

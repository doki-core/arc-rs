use serde::Serialize;
use serde_json::Value;

use voml_types::{Von, VonSerializer};

use crate::{TestEnum, TestStruct, TestTuple, TestTupleUnit, TestUnit};

#[test]
fn test_atom() {
    let von = TestUnit.serialize(VonSerializer {}).unwrap();
    assert_eq!(von, Von::list("TestUnit", vec![]));
    let von = TestTupleUnit().serialize(VonSerializer {}).unwrap();
    assert_eq!(von, Von::list("TestTupleUnit", vec![]));
    // let von = TestStructUnit {}.serialize(VonSerializer {}).unwrap();
    // assert_eq!(von, Von::list("TestStructUnit", vec![]));

    println!("{:#?}", TestTuple(0, 1, 2).serialize(VonSerializer {}).unwrap());
    println!("{:#?}", TestStruct { a: 0, b: 1, c: 2 }.serialize(VonSerializer {}).unwrap());
}

#[test]
fn test_atom2() {
    println!("{:#?}", TestEnum::Nothing.serialize(VonSerializer {}).unwrap());
    println!("{:#?}", TestEnum::Something(true).serialize(VonSerializer {}).unwrap());
}

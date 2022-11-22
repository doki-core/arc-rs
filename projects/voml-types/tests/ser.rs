use serde::Serialize;

use voml_types::VonSerializer;

use crate::TestUnit;

#[test]
fn test_atom() {
    println!("{:#?}", TestUnit.serialize(VonSerializer {}).unwrap())
}

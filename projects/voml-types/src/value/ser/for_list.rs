use super::*;

pub struct VList {
    name: String,
    vec: Vec<Von>,
}
impl SerializeSeq for VList {}
impl SerializeTuple for VList {}
impl SerializeTupleStruct for VList {}
impl SerializeTupleVariant for VList {}

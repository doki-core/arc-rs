use super::*;

pub struct VDict {
    name: String,
    map: IndexMap<String, Von>,
}

impl SerializeMap for VDict {}
impl SerializeStruct for VDict {}
impl SerializeStructVariant for VDict {}

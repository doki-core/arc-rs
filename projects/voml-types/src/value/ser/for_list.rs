use super::*;
use serde::Serialize;
use std::convert::TryFrom;
use voml_collection::List;

pub struct VList {
    pub name: String,
    pub vec: Vec<Von>,
}

impl SerializeSeq for VList {
    type Ok = Von;
    type Error = VError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let item = value.serialize(VonSerializer {})?;
        self.vec.push(item);
        Ok(())
    }

    fn end(self) -> VResult<Self::Ok> {
        Ok(Von::List(Box::new(List { hint: self.name, list: self.vec })))
    }
}

impl SerializeTuple for VList {
    type Ok = Von;
    type Error = VError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl SerializeTupleStruct for VList {
    type Ok = Von;
    type Error = VError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl SerializeTupleVariant for VList {
    type Ok = Von;
    type Error = VError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

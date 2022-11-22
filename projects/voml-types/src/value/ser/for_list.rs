use std::convert::TryFrom;

use serde::Serialize;

use voml_collection::List;

use super::*;

pub struct VList {
    pub name: String,
    pub vec: Vec<Von>,
}

impl VList {
    fn to_list(self) -> Von {
        Von::Table(Box::new(List { hint: self.name, list: self.vec }))
    }
}

impl SerializeSeq for VList {
    type Ok = Von;
    type Error = VError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized,
        T: Serialize,
    {
        let item = value.serialize(VonSerializer {})?;
        self.vec.push(item);
        Ok(())
    }

    fn end(self) -> VResult<Self::Ok> {
        Ok(self.to_list())
    }
}

impl SerializeTuple for VList {
    type Ok = Von;
    type Error = VError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized,
        T: Serialize,
    {
        let item = value.serialize(VonSerializer {})?;
        self.vec.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.to_list())
    }
}

impl SerializeTupleStruct for VList {
    type Ok = Von;
    type Error = VError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized,
        T: Serialize,
    {
        let item = value.serialize(VonSerializer {})?;
        self.vec.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.to_list())
    }
}

impl SerializeTupleVariant for VList {
    type Ok = Von;
    type Error = VError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized,
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.to_list())
    }
}

use serde::Serialize;

use super::*;

impl SerializeSeq for STable {
    type Ok = Von;
    type Error = VError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.push_serialize(value)
    }

    #[inline]
    fn end(self) -> VResult<Self::Ok> {
        Ok(self.to_table())
    }
}

impl SerializeTuple for STable {
    type Ok = Von;
    type Error = VError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.push_serialize(value)
    }

    #[inline]
    fn end(self) -> VResult<Self::Ok> {
        Ok(self.to_table())
    }
}

impl SerializeTupleStruct for STable {
    type Ok = Von;
    type Error = VError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.push_serialize(value)
    }

    #[inline]
    fn end(self) -> VResult<Self::Ok> {
        Ok(self.to_table())
    }
}

impl SerializeTupleVariant for STable {
    type Ok = Von;
    type Error = VError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.push_serialize(value)
    }

    #[inline]
    fn end(self) -> VResult<Self::Ok> {
        Ok(self.to_table())
    }
}

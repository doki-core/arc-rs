use serde::{ser::Error, Serialize};

use super::*;

impl SerializeMap for STable {
    type Ok = Von;
    type Error = VError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // let key = next_key.take();
        // // Panic because this indicates a bug in the program rather than an
        // // expected failure.
        // let key = key.expect("serialize_value called before serialize_key");
        // map.insert(key, tri!(to_value(value)));
        Ok(())
    }

    #[inline]
    fn end(self) -> VResult<Self::Ok> {
        Ok(self.to_table())
    }
}

impl SerializeStruct for STable {
    type Ok = Von;
    type Error = VError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized,
        T: Serialize,
    {
        let mut value = value.serialize(VonSerializer {})?;
        match self.map.insert(key.to_string(), value) {
            None => Ok(()),
            Some(_) => Err(VError::custom("redundant field")),
        }
    }
    #[inline]
    fn end(self) -> VResult<Self::Ok> {
        Ok(self.to_table())
    }
}

impl SerializeStructVariant for STable {
    type Ok = Von;
    type Error = VError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized,
        T: Serialize,
    {
        todo!()
    }
    #[inline]
    fn end(self) -> VResult<Self::Ok> {
        Ok(self.to_table())
    }
}

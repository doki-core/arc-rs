use num::ToPrimitive;
use serde::{ser::Error, Serialize};
use std::mem::take;

use super::*;

impl SerializeMap for STable {
    type Ok = Von;
    type Error = VError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = self.serialize(key)?;
        match key {
            Von::Number(v) => match v.to_usize() {
                None => {}
                Some(s) => return Ok(self.next_key = SKey::List(s)),
            },
            Von::String(s) => return Ok(self.next_key = SKey::Dict(s.text)),
            _ => {}
        }
        Err(VError::custom(
            "Table keys must be strings, if you really need non-string keys, consider using `serde_with::serde_as`",
        ))
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = value.serialize(self.serializer)?;
        match take(&mut self.next_key) {
            SKey::None => Err(VError::custom("Could not find next key")),
            SKey::List(v) => {
                self.vec.insert(v, value);
                Ok(())
            }
            SKey::Dict(v) => {
                self.map.insert(v, value);
                Ok(())
            }
        }
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
        T: Serialize + ?Sized,
    {
        match self.map.insert(key.to_string(), self.serialize(value)?) {
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
        T: Serialize + ?Sized,
    {
        match self.map.insert(key.to_string(), self.serialize(value)?) {
            None => Ok(()),
            Some(_) => Err(VError::custom("redundant field")),
        }
    }
    #[inline]
    fn end(self) -> VResult<Self::Ok> {
        Ok(self.to_table())
    }
}

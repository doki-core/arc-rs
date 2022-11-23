use num::ToPrimitive;
use serde::{ser::Error, Serialize};

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
            Von::Number(v) => {
                match v.to_usize() {
                    None => {}
                    Some(s) => {
                        self.key = Some(s);

                        self.vec.insert(s, key);
                        return Ok(());
                    }
                }

                todo!()
            }
            Von::String(_) => {
                todo!()
            }
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

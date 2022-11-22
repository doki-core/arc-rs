use serde::{ser::Error, Serialize};

use voml_collection::Dict;

use super::*;

pub struct VDict {
    pub name: String,
    pub map: IndexMap<String, Von>,
}

impl VDict {
    #[inline]
    fn to_dict(self) -> Von {
        Von::Dict(Box::new(Dict { hint: self.name, dict: self.map }))
    }
}

impl SerializeMap for VDict {
    type Ok = Von;
    type Error = VError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        match key.serialize(VonSerializer {})? {
            Von::Boolean(_) => {
                todo!()
            }
            Von::Integer(_) => {
                todo!()
            }
            Von::Number(_) => {
                todo!()
            }
            Von::String(s) => {
                println!("{:#?}", s);
                todo!()
            }
            Von::Binary(_) => {
                todo!()
            }
            Von::List(_) => {
                todo!()
            }
            Von::Dict(_) => {
                todo!()
            }
        }
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
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.to_dict())
    }
}

impl SerializeStruct for VDict {
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
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.to_dict())
    }
}

impl SerializeStructVariant for VDict {
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
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.to_dict())
    }
}

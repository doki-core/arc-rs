
use serde::{ser::Error, Serializer};
use voml_collection::BigDecimal;

use super::*;

impl SerializeDecimalToInteger {
    pub fn extra_integer(&self, von: Von) -> VResult<BigDecimal> {
        let number = match von {
            Von::Number(v) => {
                v.value
            }
           _=> {
               type_mismatch("u64", self.von.type_name())
           }
        };
        
        match self {
            SerializeDecimalToInteger::Prohibit => {
                
            }
            SerializeDecimalToInteger::Round => {
                
            }
            SerializeDecimalToInteger::Ceil => {
                
            }
            SerializeDecimalToInteger::Floor => {
                
            }
        }
        
        let number = match self.von {
            Von::Number(v) => match self.decimal_to_integer {
                SerializeDecimalToInteger::Prohibit => {

                }
                SerializeDecimalToInteger::Round => {

                }
                SerializeDecimalToInteger::Ceil => {}
                SerializeDecimalToInteger::Floor => {}
            },
            _ => return type_mismatch("u64", self.von.type_name()),
        };
    }
}

impl Serializer for Object2Von {
    type Ok = Von;
    type Error = VError;

    type SerializeSeq = STable;
    type SerializeTuple = STable;
    type SerializeTupleStruct = STable;
    type SerializeTupleVariant = STable;
    type SerializeMap = STable;
    type SerializeStruct = STable;
    type SerializeStructVariant = STable;

    #[inline]
    fn serialize_bool(self, value: bool) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }
    #[inline]
    fn serialize_i64(self, value: i64) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }
    #[inline]
    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }
    #[inline]
    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_char(self, value: char) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> VResult<Self::Ok> {
        Ok(Von::from(value))
    }

    fn serialize_bytes(self, value: &[u8]) -> VResult<Self::Ok> {
        Ok(Von::Binary(Box::new(Bytes::from(value))))
    }

    #[inline]
    fn serialize_unit(self) -> VResult<Self::Ok> {
        // Ok(SahaNode::null())
        todo!()
    }

    #[inline]
    fn serialize_unit_struct(self, name: &'static str) -> VResult<Self::Ok> {
        Ok(Von::list(name.to_string(), vec![]))
    }

    #[inline]
    #[allow(unused_variables)]
    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> VResult<Self::Ok> {
        Ok(Von::String(Box::new(Text { hint: name.to_string(), text: variant.to_string() })))
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> VResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        todo!()
        // value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> VResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        todo!()
        // let mut values = Map::new();
        // values.insert(String::from(variant), tri!(to_value(value)));
        // Ok(Value::Object(values))
    }

    #[inline]
    fn serialize_none(self) -> VResult<Self::Ok> {
        todo!()
        // self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> VResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        todo!()
        // value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> VResult<Self::SerializeSeq> {
        Ok(STable {
            name: "".to_string(),
            vec: Vec::with_capacity(len.unwrap_or(0)),
            map: Default::default(),
            serializer: self,
            next_key: SKey::None,
        })
    }
    #[inline]
    fn serialize_tuple(self, len: usize) -> VResult<Self::SerializeTuple> {
        Ok(STable {
            name: "".to_string(),
            vec: Vec::with_capacity(len),
            map: Default::default(),
            serializer: self,
            next_key: SKey::None,
        })
    }
    #[inline]
    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> VResult<Self::SerializeTupleStruct> {
        Ok(STable {
            name: name.to_string(),
            vec: Vec::with_capacity(len),
            map: Default::default(),
            serializer: self,
            next_key: SKey::None,
        })
    }

    #[inline]
    #[allow(unused_variables)]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> VResult<Self::SerializeTupleVariant> {
        // println!("name: {name}");
        // println!("variant_index: {variant_index}");
        // println!("variant: {variant}");
        // println!("len: {len}");
        Ok(STable {
            name: name.to_string(),
            vec: Vec::with_capacity(len),
            map: Default::default(),
            serializer: self,
            next_key: SKey::None,
        })
    }
    #[inline]
    fn serialize_map(self, length: Option<usize>) -> VResult<Self::SerializeMap> {
        Ok(STable {
            name: "".to_string(),
            vec: vec![],
            map: IndexMap::with_capacity(length.unwrap_or(0)),
            serializer: self,
            next_key: SKey::None,
        })
    }
    #[inline]
    fn serialize_struct(self, name: &'static str, length: usize) -> VResult<Self::SerializeStruct> {
        Ok(STable {
            name: name.to_string(),
            vec: vec![],
            map: IndexMap::with_capacity(length),
            serializer: self,
            next_key: SKey::None,
        })
    }

    #[inline]
    #[allow(unused_variables)]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> VResult<Self::SerializeStructVariant> {
        // println!("name: {name}");
        // println!("variant_index: {variant_index}");
        // println!("variant: {variant}");
        // println!("len: {len}");

        Ok(STable {
            name: name.to_string(),
            vec: Vec::with_capacity(len),
            map: Default::default(),
            serializer: self,
            next_key: SKey::None,
        })
    }
    #[inline]
    fn collect_str<T>(self, value: &T) -> VResult<Self::Ok>
    where
        T: ?Sized + Display,
    {
        Ok(Von::from(value.to_string()))
    }
}

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

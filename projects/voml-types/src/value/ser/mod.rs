use std::fmt::Display;

use indexmap::IndexMap;
use serde::{
    ser::{
        Error, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct,
        SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use voml_collection::{Bytes, Text};

use crate::{Dict, List, Table, VError, VResult, Von};

mod for_dict;
mod for_list;

///
#[derive(Debug, Copy, Clone)]
pub struct VonSerializer {
    enumeration_as_id: bool,
}

impl Default for VonSerializer {
    fn default() -> Self {
        Self { enumeration_as_id: false }
    }
}

impl Error for VError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

pub struct STable {
    pub name: String,
    pub vec: List,
    pub map: Dict,
    pub serializer: VonSerializer,
}

impl STable {
    fn to_table(self) -> Von {
        Von::Table(Box::new(Table { hint: self.name, list: self.vec, dict: self.map }))
    }
    fn serialize<T>(&self, value: &T) -> VResult<Von>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self.serializer)
    }
    fn push_serialize<T>(&mut self, value: &T) -> VResult<()>
    where
        T: Serialize + ?Sized,
    {
        Ok(self.vec.push(value.serialize(self.serializer)?))
    }
}

impl Serializer for VonSerializer {
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
        })
    }
    #[inline]
    fn serialize_tuple(self, len: usize) -> VResult<Self::SerializeTuple> {
        Ok(STable { name: "".to_string(), vec: Vec::with_capacity(len), map: Default::default(), serializer: self })
    }
    #[inline]
    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> VResult<Self::SerializeTupleStruct> {
        Ok(STable { name: name.to_string(), vec: Vec::with_capacity(len), map: Default::default(), serializer: self })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> VResult<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, length: Option<usize>) -> VResult<Self::SerializeMap> {
        Ok(STable { name: "".to_string(), vec: vec![], map: IndexMap::with_capacity(length.unwrap_or(0)), serializer: self })
    }

    fn serialize_struct(self, name: &'static str, length: usize) -> VResult<Self::SerializeStruct> {
        Ok(STable { name: name.to_string(), vec: vec![], map: IndexMap::with_capacity(length), serializer: self })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> VResult<Self::SerializeStructVariant> {
        todo!()
        // Ok(MapVariant { name: String::from(variant), map: Map::new() })
    }

    fn collect_str<T>(self, value: &T) -> VResult<Self::Ok>
    where
        T: ?Sized + Display,
    {
        Ok(Von::from(value.to_string()))
    }
}

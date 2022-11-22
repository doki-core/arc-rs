use indexmap::IndexMap;
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct,
        SerializeTupleVariant,
    },
    Serializer,
};

use crate::{VError, VResult, Von, VonSerializer};

pub use self::{for_dict::VDict, for_list::VList};

mod for_dict;
mod for_list;

impl Serializer for VonSerializer {
    type Ok = Von;
    type Error = VError;

    type SerializeSeq = VList;
    type SerializeTuple = VList;
    type SerializeTupleStruct = VList;
    type SerializeTupleVariant = VList;
    type SerializeMap = VDict;
    type SerializeStruct = VDict;
    type SerializeStructVariant = VDict;

    #[inline]
    fn serialize_bool(self, value: bool) -> VResult<Self::Ok> {
        Ok(SahaNode::boolean(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> VResult<Self::Ok> {
        Ok(SahaNode::number(Decimal::from(value)))
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> VResult<Self::Ok> {
        Ok(SahaNode::number(Decimal::from(value)))
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> VResult<Self::Ok> {
        Ok(SahaNode::number(Decimal::from(value)))
    }
    #[inline]
    fn serialize_i64(self, value: i64) -> VResult<Self::Ok> {
        Ok(SahaNode::number(Decimal::from(value)))
    }
    #[inline]
    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        match Decimal::from_i128(value) {
            Some(s) => Ok(SahaNode::number(s)),
            None => Err(QError::syntax_error(format!("{value} can not cast to `Number`"))),
        }
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> VResult<Self::Ok> {
        Ok(SahaNode::number(Decimal::from(value)))
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> VResult<Self::Ok> {
        Ok(SahaNode::number(Decimal::from(value)))
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> VResult<Self::Ok> {
        Ok(SahaNode::number(Decimal::from(value)))
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> VResult<Self::Ok> {
        Ok(SahaNode::number(Decimal::from(value)))
    }
    #[inline]
    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        match Decimal::from_u128(value) {
            Some(s) => Ok(SahaNode::number(s)),
            None => Err(QError::syntax_error(format!("{value} can not cast to `Number`"))),
        }
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> VResult<Self::Ok> {
        match Decimal::from_f32(value) {
            Some(s) => Ok(SahaNode::number(s)),
            None => Err(QError::syntax_error(format!("{value} can not cast to `Number`"))),
        }
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> VResult<Self::Ok> {
        match Decimal::from_f64(value) {
            Some(s) => Ok(SahaNode::number(s)),
            None => Err(QError::syntax_error(format!("{value} can not cast to `Number`"))),
        }
    }

    #[inline]
    fn serialize_char(self, value: char) -> VResult<Self::Ok> {
        todo!("{value}")
    }

    #[inline]
    fn serialize_str(self, value: &str) -> VResult<Self::Ok> {
        todo!("{value}")
    }

    fn serialize_bytes(self, value: &[u8]) -> VResult<Self::Ok> {
        let vec = value.iter().map(|&b| Value::Number(b.into())).collect();
        Ok(Value::Array(vec))
    }

    #[inline]
    fn serialize_unit(self) -> VResult<Self::Ok> {
        Ok(SahaNode::null())
    }

    #[inline]
    #[allow(unused_variables)]
    fn serialize_unit_struct(self, name: &'static str) -> VResult<Self::Ok> {
        self.serialize_unit()
    }

    #[inline]
    #[allow(unused_variables)]
    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> VResult<Self::Ok> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> VResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
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
        let mut values = Map::new();
        values.insert(String::from(variant), tri!(to_value(value)));
        Ok(Value::Object(values))
    }

    #[inline]
    fn serialize_none(self) -> VResult<Self::Ok> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> VResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> VResult<Self::SerializeSeq> {
        Ok(SerializeVec { vec: Vec::with_capacity(len.unwrap_or(0)) })
    }

    fn serialize_tuple(self, len: usize) -> VResult<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> VResult<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SeqVariant { name: String::from(variant), vec: Vec::with_capacity(len) })
    }

    fn serialize_map(self, len: Option<usize>) -> VResult<Self::SerializeMap> {
        Ok(MapEx { map: Map::new(), next_key: None })
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> VResult<Self::SerializeStruct> {
        match name {
            _ => self.serialize_map(Some(len)),
        }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> VResult<Self::SerializeStructVariant> {
        Ok(MapVariant { name: String::from(variant), map: Map::new() })
    }

    fn collect_str<T>(self, value: &T) -> VResult<Self::Ok>
    where
        T: ?Sized + Display,
    {
        Ok(Value::String(value.to_string()))
    }
}

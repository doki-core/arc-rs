use std::fmt::Display;

use num::ToPrimitive;
use serde::{
    de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Serialize,
};

use crate::{SerializeDecimalToInteger, Serializer, VError, VErrorKind, VResult, Von};

mod methods;

impl Serializer {
    /// Convert von object to other types
    ///
    /// # Arguments
    ///
    /// * `value`:
    ///
    /// returns: Result<Von, VError>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::ObjectSerializer;
    /// ```
    pub fn serialize_object<'de, T: Deserialize<'de>>(&self, value: Von) -> VResult<T> {
        T::deserialize(Von2Object { von: value, decimal_to_integer: self.decimal_to_integer })
    }
}

impl Error for VError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        VError { kind: Box::new(VErrorKind::Custom(msg.to_string())), level: Default::default(), file: Default::default() }
    }
}

pub struct Von2Object {
    pub von: Von,
    pub decimal_to_integer: SerializeDecimalToInteger,
}

impl Von2Object {}

impl<'de> serde::Deserializer<'de> for Von2Object {
    type Error = VError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.von {
            Von::Boolean(v) => visitor.visit_bool(v),
            _ => type_mismatch("bool", self.von.type_name()),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    #[inline]
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        
        
        let number = match self.von {
            Von::Number(v) => match self.decimal_to_integer {
                SerializeDecimalToInteger::Prohibit => {
                    if !v.is_integer() {
                        custom_error("TODO")
                    }
                    else {
                        v.value
                    }
                }
                SerializeDecimalToInteger::Round => {
                    
                }
                SerializeDecimalToInteger::Ceil => {}
                SerializeDecimalToInteger::Floor => {}
            },
            _ => return type_mismatch("u64", self.von.type_name()),
        };
        
        
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.von {
            Von::String(v) => {
                let mut chars = v.text.chars().peekable();
                match chars.next() {
                    Some(s) => match chars.peek() {
                        Some(_) => custom_error("Too much characters"),
                        None => visitor.visit_char(s),
                    },
                    None => custom_error("Too less characters"),
                }
            }
            _ => type_mismatch("char", self.von.type_name()),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match &self.von {
            Von::String(v) => visitor.visit_str(&v.text),
            _ => type_mismatch("&str", self.von.type_name()),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.von {
            Von::String(v) => visitor.visit_string(v.text),
            _ => type_mismatch("String", self.von.type_name()),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        todo!()
    }

    // fn __deserialize_content<V>(self, _: serde::actually_private::T, visitor: V) -> Result<Content<'de>, Self::Error> where V: Visitor<'de, Value=Content<'de>> {
    //     todo!()
    // }
}

fn type_mismatch<T>(expected: &str, actual: &str) -> VResult<T> {
    Err(VError::custom(format!("Expected type `{expected}`, actual type `{actual}`",)))
}

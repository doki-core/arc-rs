use std::{fmt::Display, mem::take};

use indexmap::IndexMap;
use num::ToPrimitive;
use serde::{
    ser::{
        Error, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct,
        SerializeTupleVariant,
    },
    Serialize,
};

use voml_collection::{Bytes, Text};

use crate::{Deserializer, Dict, List, Table, VError, VErrorKind, VResult, Von};

mod for_dict;
mod object2von;

impl Deserializer {
    /// Convert other objects to von object
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
    pub fn deserializer_object<T: Serialize>(&self, value: &T) -> VResult<Von> {
        let ser = Object2Von { enumeration_as_id: false };
        value.serialize(ser)
    }
}

/// Convert other objects to von object
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
#[derive(Debug, Copy, Clone)]
pub struct Serializer {
    /// Convert other objects to von object
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
    pub decimal_to_integer: SerializeDecimalToInteger,
}

#[derive(Copy, Clone)]
pub struct Object2Von {
    pub enumeration_as_id: bool,
}

impl Default for Object2Von {
    fn default() -> Self {
        Self { enumeration_as_id: false }
    }
}

impl Error for VError {
    fn custom<T: Display>(msg: T) -> Self {
        VError { kind: Box::new(VErrorKind::Custom(msg.to_string())), level: Default::default() }
    }
}

pub struct STable {
    pub name: String,
    pub vec: List,
    pub map: Dict,
    pub serializer: Object2Von,
    pub next_key: SKey,
}

pub enum SKey {
    None,
    List(usize),
    Dict(String),
}

impl Default for SKey {
    fn default() -> Self {
        SKey::None
    }
}

/// Convert other objects to von object
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
#[derive(Debug, Copy, Clone)]
pub enum SerializeDecimalToInteger {
    Prohibit,
    /// Convert other objects to von object
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
    Round,
    /// Convert other objects to von object
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
    Ceil,
    /// Convert other objects to von object
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
    Floor,
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

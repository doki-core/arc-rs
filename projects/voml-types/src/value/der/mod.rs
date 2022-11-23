use std::fmt::Display;

use serde::{de::Error, Serialize};

use crate::{value::ser::Object2Von, VError, VErrorKind, VResult, Von};

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
#[derive(Copy, Clone, Debug, Default)]
pub struct Deserializer {
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
    pub enumeration_as_integer: bool,
}

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
    pub fn deserialize_object<T: Serialize>(&self, value: &T) -> VResult<Von> {
        let ser = Object2Von { enumeration_as_id: false };
        value.serialize(ser)
    }
}

fn test() {}

impl Error for VError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        VError { kind: Box::new(VErrorKind::Custom(msg.to_string())), level: Default::default(), file: Default::default() }
    }
}

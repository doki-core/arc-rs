use std::fmt::{Debug, Formatter};

use indexmap::IndexMap;
use num::{FromPrimitive, ToPrimitive};

use voml_collection::{BigDecimal, Bytes, Number, Text};
pub mod der;
mod display;
mod number;
pub mod ser;
mod text;

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
pub type List = Vec<Von>;
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
pub type Dict = IndexMap<String, Von>;
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
pub type Table = voml_collection::Table<Von>;

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
#[derive(PartialEq, Eq)]
pub enum Von {
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
    Boolean(bool),
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
    Number(Box<Number>),
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
    String(Box<Text>),
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
    Binary(Box<Bytes>),
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
    Table(Box<Table>),
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
pub struct Serializer {}

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

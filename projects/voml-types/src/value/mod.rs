use std::fmt::{Debug, Formatter};

use indexmap::IndexMap;
use num::{FromPrimitive, ToPrimitive};

use voml_collection::{BigDecimal, Bytes, Number, Text};
pub mod der;
mod display;
mod number;
pub mod ser;
mod text;

///
pub type List = Vec<Von>;
///
pub type Dict = IndexMap<String, Von>;
///
pub type Table = voml_collection::Table<Von>;

/// Represents an valid [VON]() value.
///
/// See the [`serde_json::value` module documentation](self) for usage examples.
#[derive(PartialEq, Eq)]
pub enum Von {
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use von::von;
    /// #
    /// let v = von!(true);
    /// ```
    Boolean(bool),
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Number(Box<Number>),
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    String(Box<Text>),
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Binary(Box<Bytes>),
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use von::von;
    /// #
    /// let v = von!(true);
    /// ```
    Table(Box<Table>),
}

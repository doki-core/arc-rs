use std::fmt::{Debug, Display, Formatter};

use num::{FromPrimitive, ToPrimitive};

use serde::{ser::SerializeSeq, Serialize, Serializer};
use voml_collection::{Bytes, Dict, Integer, List, Number, Text};

mod der;
mod display;
mod number;
mod ser;
mod text;

///
pub struct VonSerializer;

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
    List(Box<List<Von>>),
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Dict(Box<Dict<Von>>),
}

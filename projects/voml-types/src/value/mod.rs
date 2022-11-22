use std::fmt::{Debug, Formatter};

use num::{FromPrimitive, ToPrimitive};

use voml_collection::{Bytes, Decimal, Dict, Integer, List, Text};

mod der;
mod display;
mod number;
mod ser;

///
pub struct VonSerializer {}

/// Represents an valid [VON]() value.
///
/// See the [`serde_json::value` module documentation](self) for usage examples.
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
    /// # use von::von;
    /// #
    /// let v = von!(true);
    /// ```
    Integer(Box<Integer>),
    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use von::von;
    /// #
    /// let v = von!(true);
    /// ```
    Decimal(Box<Decimal>),
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
}

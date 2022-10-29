use std::fmt::Formatter;

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use super::*;

impl<'de> Deserialize<'de> for VonNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Text {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TextVisitor {})
    }
}
struct TextVisitor {}

impl<'de> Visitor<'de> for TextVisitor {
    type Value = Text;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except a `String` or `Text` struct")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Text {
            hint: "".to_string(),
            value: v.to_string(),
        })
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, D::Error>
    where
        A: MapAccess<'de>,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for List {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

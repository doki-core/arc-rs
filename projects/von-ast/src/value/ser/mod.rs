use serde::{ser::SerializeStruct, Serialize, Serializer};

use super::*;

impl Serialize for VonNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("VonNode", 2)?;
        // s.serialize_field("hint", &self.hint)?;
        // s.serialize_field("value", &self.value)?;
        s.end()
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = serializer.serialize_struct("VonNumber", 2)?;
        s.serialize_field("hint", &self.hint)?;
        s.serialize_field("value", &self.value.to_string())?;
        s.end()
    }
}

impl Serialize for Text {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("VonText", 2)?;
        s.serialize_field("hint", &self.hint)?;
        s.serialize_field("value", &self.value)?;
        s.end()
    }
}

impl Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("VonDict", 2)?;
        s.serialize_field("hint", &self.hint)?;
        s.serialize_field("value", &self.dict)?;
        s.end()
    }
}

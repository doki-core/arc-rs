use std::fmt::{Debug, Formatter};

use crate::BigFloatNumber;
use num::FromPrimitive;
use serde::{Deserialize, Serialize};
mod primitive;

/// An arbitrary-precision decimal with a unit
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decimal {
    /// The unit of the decimal
    pub hint: String,
    /// The value of the decimal
    pub value: BigFloatNumber,
}

impl Debug for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decimal").field("hint", &self.hint).field("value", &self.value.to_string()).finish()
    }
}

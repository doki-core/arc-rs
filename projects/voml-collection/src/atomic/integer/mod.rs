use dashu::integer::IBig;
use serde::{Deserialize, Serialize};

mod cmp;
mod primitive;

/// An arbitrary-precision integer with a unit
#[derive(Clone, Debug, Default, Eq, Serialize, Deserialize)]
pub struct Integer {
    /// The unit of this integer
    pub hint: String,
    /// The value of this integer
    pub value: IBig,
}

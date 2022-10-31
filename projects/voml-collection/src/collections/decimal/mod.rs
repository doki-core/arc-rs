use bigdecimal::BigDecimal;
use num::FromPrimitive;
use serde::{Deserialize, Serialize};

mod primitive;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decimal {
    pub hint: String,
    pub value: BigDecimal,
}

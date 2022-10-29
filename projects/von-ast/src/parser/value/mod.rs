use super::*;
use crate::parser::von::SpecialNode;

impl NumNode {
    pub fn into_von(self) -> Number {
        Number { hint: "".to_string(), value: Default::default() }
    }
}

impl SpecialNode {
    pub fn into_von(self) -> Number {
        Number { hint: "".to_string(), value: Default::default() }
    }
}

use super::*;

impl Deref for Number {
    type Target = NumberKind;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Number {
    pub fn get_handler(&self) -> Option<String> {
        self.handler.to_owned()
    }
}

impl NumberKind {
    pub fn is_integer(&self) -> bool {
        match self {
            NumberKind::InlineInteger(_) | NumberKind::BigInteger(_) => true,
            NumberKind::InlineDecimal(_) | NumberKind::BigDecimal(_) => false,
        }
    }
    pub fn is_decimal(&self) -> bool {
        match self {
            NumberKind::InlineInteger(_) | NumberKind::BigInteger(_) => false,
            NumberKind::InlineDecimal(_) | NumberKind::BigDecimal(_) => true,
        }
    }
    pub fn is_zero(&self) -> bool {
        match self {
            NumberKind::InlineInteger(n) => n.is_zero(),
            NumberKind::InlineDecimal(n) => n.is_zero(),
            NumberKind::BigInteger(n) => n.is_zero(),
            NumberKind::BigDecimal(n) => n.is_zero(),
        }
    }
}
impl NumberKind {
    pub fn as_index(&self) -> Option<usize> {
        usize::try_from(self.clone()).ok()
    }
}

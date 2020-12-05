use super::*;
use std::cmp::Ordering;

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            Some(s) => write!(f, "{}{}", self.value, s),
            None => write!(f, "{}", self.value),
        }
    }
}

impl Display for NumberKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            NumberKind::InlineInteger(n) => write!(f, "{}", n),
            NumberKind::InlineDecimal(n) => write!(f, "{}", n),
            NumberKind::BigInteger(n) => write!(f, "{}", n),
            NumberKind::BigDecimal(n) => write!(f, "{}", n),
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Self {
            handler: None,
            value: NumberKind::default(),
        }
    }
}

impl Default for NumberKind {
    fn default() -> Self {
        Self::InlineInteger(0)
    }
}

impl<T> From<T> for Number
where
    T: Into<NumberKind>,
{
    fn from(v: T) -> Self {
        Self {
            handler: None,
            value: v.into(),
        }
    }
}

impl Eq for NumberKind {}

impl PartialEq for NumberKind {
    fn eq(&self, other: &Self) -> bool {
        let lhs = f64::from(self);
        let rhs = f64::from(other);
        lhs == rhs
    }
}

impl PartialOrd for NumberKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs = f64::from(self);
        let rhs = f64::from(other);
        lhs.partial_cmp(&rhs)
    }
}

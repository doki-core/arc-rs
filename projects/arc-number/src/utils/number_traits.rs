use super::*;

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

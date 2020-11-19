use super::*;
use std::convert::TryFrom;

impl TryFrom<NumberKind> for BigInt {
    type Error = ();
    fn try_from(n: NumberKind) -> Result<Self, Self::Error> {
        match n {
            NumberKind::BigInteger(i) => match BigInt::try_from(i) {
                Ok(o) => return Ok(o),
                Err(_) => (),
            },
            NumberKind::InlineInteger(u) => match BigInt::try_from(u) {
                Ok(o) => return Ok(o),
                Err(_) => (),
            },
            _ => (),
        }
        Err(())
    }
}

// impl<T> From<NumberKind> for T
// where
//     T: TryFrom<NumberKind> + Default,
// {
//     fn from(n: NumberKind) -> Self {
//         T::try_from(n).unwrap_or_default()
//     }
// }

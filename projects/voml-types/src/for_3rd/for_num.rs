use crate::VError;
use num::{bigint::TryFromBigIntError, BigInt};
use rust_decimal::Error;

impl From<TryFromBigIntError<BigInt>> for VError {
    fn from(e: TryFromBigIntError<BigInt>) -> Self {
        Self::runtime_error(format!("{}", e))
    }
}

impl From<Error> for VError {
    fn from(e: Error) -> Self {
        Self::runtime_error(format!("{}", e))
    }
}

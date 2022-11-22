use std::fmt::{Debug, Formatter};

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

mod display;
mod primitive;

/// An arbitrary-precision integer with a unit
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Number {
    /// The unit of this number
    pub hint: String,
    /// The value of this number
    pub value: BigDecimal,
}

impl Number {
    /// is_decimal
    ///
    /// # Arguments
    ///
    /// * `b`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_collection::Number;
    /// ```
    #[inline]
    pub fn is_integer(&self) -> bool {
        self.value.is_integer()
    }

    /// is_decimal
    ///
    /// # Arguments
    ///
    /// * `b`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_collection::Number;
    /// ```
    #[inline]
    pub fn is_decimal(&self) -> bool {
        !self.is_integer()
    }
}

// impl FromStr for Number {
//     type Err = ParseBigDecimalError;
//
//     /// ```js
//     /// +1
//     /// -1
//     /// +1.0
//     /// -1.0m
//     /// +2**5   => 2 * 10^5
//     /// ```
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let (sign, rest) = match s.split_once(|c| c == '+' || c == '-') {
//             // `+1`,
//             Some(("+", rest)) => (Sign::NoSign, rest),
//             // `-1`
//             Some(("-", rest)) => (Sign::NoSign, rest),
//             _ => (Sign::NoSign, s),
//         };
//         match s.split_once("**") {
//             // `2**5   => 2*10^5`,
//             Some((lhs, rhs)) => {
//                 let lhs = match BigDecimal::from_str(lhs) {
//                     Ok(o) => {o}
//                     Err(_) => {
//                         ParseIntError::from()
//                     }
//                 };
//                 match rhs.split_once(|c| !c.is_ascii_digit()) {
//                     Some(_) => {}
//                     None => {}
//                 }
//                 let exp = u32::from_str()?;
//
//
//                 10.pow()
//             }
//             _ => match rest.split_once(|c| !c.is_ascii_digit()) {
//                 None => {}
//                 Some(_) => {}
//             },
//         };
//
//         let value = BigDecimal::from_str_radix(s, 10)?;
//         Ok(Number { hint: "".to_string(), value })
//     }
// }

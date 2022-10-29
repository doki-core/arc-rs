use std::cmp::Ordering;
use std::ops::{Add, Mul};
use std::ops::{Div, Neg, Rem, Sub};

use bigdecimal::ParseBigDecimalError;
use num::traits::NumOps;
use num::Num;
use num::{NumCast, ToPrimitive};
use num::{One, Zero};

use super::*;

mod arithmetic;

impl Number {
    #[inline]
    pub fn is_integer(&self) -> bool {
        self.value.is_integer()
    }
}

impl Zero for Number {
    fn zero() -> Self {
        Self {
            hint: String::new(),
            value: BigDecimal::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl One for Number {
    fn one() -> Self {
        Self {
            hint: String::new(),
            value: BigDecimal::one(),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.hint.eq(&other.hint) && self.value.eq(&other.value)
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hint.eq(&other.hint) {
            true => self.value.partial_cmp(&other.value),
            false => None,
        }
    }
}

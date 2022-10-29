use std::ops::{Add, Mul};

use num::{One, Zero};

use super::*;

impl Number {
    #[inline]
    pub fn is_integer(&self) -> bool {
        self.value.is_integer()
    }
}

impl Add<Self> for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Zero for Number {
    fn zero() -> Self {
        Self {
            hint: None,
            value: BigDecimal::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl Mul<Self> for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl One for Number {
    fn one() -> Self {
        Self {
            hint: None,
            value: BigDecimal::one(),
        }
    }
}

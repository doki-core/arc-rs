use num::Signed;

use super::*;

impl Add<Self> for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { hint: self.hint, value: self.value.add(rhs.value) }
    }
}

impl Sub<Self> for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { hint: self.hint, value: self.value.add(rhs.value) }
    }
}

impl Mul<Self> for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { hint: self.hint, value: self.value.mul(rhs.value) }
    }
}

impl Div<Self> for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { hint: self.hint, value: self.value.div(rhs.value) }
    }
}

impl Rem<Self> for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self { hint: self.hint, value: self.value.rem(rhs.value) }
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { hint: self.hint, value: self.value.neg() }
    }
}

impl Signed for Number {
    fn abs(&self) -> Self {
        Self { hint: self.hint.clone(), value: self.value.abs() }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self { hint: self.hint.clone(), value: self.value.abs_sub(&other.value) }
    }

    fn signum(&self) -> Self {
        Self { hint: self.hint.clone(), value: self.value.signum() }
    }

    fn is_positive(&self) -> bool {
        self.value.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.value.is_negative()
    }
}

// impl NumOps for Number {}

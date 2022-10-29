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

// impl NumOps for Number {}

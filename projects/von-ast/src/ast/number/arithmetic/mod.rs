use super::*;

impl Add<Self> for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            hint: self.hint,
            value: self.value.add(rhs.value),
        }
    }
}

impl Sub<Self> for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            hint: self.hint,
            value: self.value.add(rhs.value),
        }
    }
}

impl Mul<Self> for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            hint: self.hint,
            value: self.value.mul(rhs.value),
        }
    }
}

impl Div<Self> for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            hint: self.hint,
            value: self.value.div(rhs.value),
        }
    }
}

impl Rem<Self> for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            hint: self.hint,
            value: self.value.rem(rhs.value),
        }
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            hint: self.hint,
            value: self.value.neg(),
        }
    }
}

impl Num for Number {
    type FromStrRadixErr = ParseBigDecimalError;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let dec = BigDecimal::from_str_radix(str, radix)?;
        Ok(Self {
            hint: "".to_string(),
            value: dec,
        })
    }
}

impl NumOps for Number {}

impl NumCast for Number {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        todo!()
    }
}

impl ToPrimitive for Number {
    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    fn to_i128(&self) -> Option<i128> {
        self.value.to_i128()
    }

    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }

    fn to_u128(&self) -> Option<u128> {
        self.value.to_u128()
    }
    fn to_f64(&self) -> Option<f64> {
        self.value.to_f64()
    }
}

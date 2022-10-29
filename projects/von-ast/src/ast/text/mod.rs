use std::ops::Add;
use num::Zero;
use super::*;

impl Number {
    pub fn is_integer(&self) -> bool {
        self.value.is_integer()
    }
}

impl Add<Self, Output=Self> for Number {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Zero for Number {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl Text {

}
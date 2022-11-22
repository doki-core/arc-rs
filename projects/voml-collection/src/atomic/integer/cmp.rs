use super::*;

impl PartialEq<usize> for Integer {
    fn eq(&self, other: &usize) -> bool {
        match self.to_usize() {
            Some(s) => s.eq(other),
            None => false,
        }
    }
}

impl PartialEq<Self> for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

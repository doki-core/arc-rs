use super::*;

impl PartialEq<usize> for Integer {
    fn eq(&self, other: &usize) -> bool {
        match self.to_usize() {
            Some(s) => s.eq(other),
            None => false,
        }
    }
}

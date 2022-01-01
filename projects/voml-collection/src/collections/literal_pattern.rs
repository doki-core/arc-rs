use super::*;
use std::{
    hash::{Hash, Hasher},
    slice::Iter,
};



impl<T> Hash for LiteralVector<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in &self.inner {
            i.value.hash(state)
        }
    }
}

impl<T> LiteralVector<T> {
    /// Returns the number of elements in the vector, also referred to as its 'length'.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns true if the vector contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> LiteralVector<T> {
    /// Returns an iterator over the slice.
    #[inline]
    pub fn iter(&self) -> LiteralPatternIter {
        LiteralPatternIter { inner: self.inner.iter() }
    }
}

/// Wrapper type of [`LiteralPattern::iter`]
pub struct LiteralPatternIter<'i> {
    inner: Iter<'i, Literal<String>>,
}

impl<'i> Iterator for LiteralPatternIter<'i> {
    type Item = &'i String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|f| &f.value)
    }
}

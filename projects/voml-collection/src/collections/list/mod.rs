use std::fmt::{Debug, Formatter, Write};

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct List<T> {
    pub hint: String,
    pub list: Vec<T>,
}

impl<T> Debug for List<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.hint.is_empty() {
            f.write_str(&self.hint)?;
            f.write_char(' ')?;
        }
        f.debug_list().entries(self.list.iter()).finish()
    }
}

impl<O, V> FromIterator<V> for List<O>
where
    O: From<V>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = V>,
    {
        let list = Vec::from_iter(iter.into_iter().map(|v| O::from(v)));
        List { hint: "".to_string(), list }
    }
}

impl<T> List<T> {
    pub fn clear(&mut self) {
        self.list.clear()
    }
    pub fn push(&mut self, value: T) {
        self.list.push(value)
    }
    pub fn get(&mut self, query: usize) -> Option<&T> {
        self.list.get(query)
    }
    pub fn get_mut(&mut self, query: usize) -> Option<&mut T> {
        self.list.get_mut(query)
    }
    pub fn extend<I, V>(&mut self, iter: I)
    where
        I: IntoIterator<Item = V>,
        T: From<V>,
    {
        self.list.extend(iter.into_iter().map(|v| T::from(v)))
    }
}

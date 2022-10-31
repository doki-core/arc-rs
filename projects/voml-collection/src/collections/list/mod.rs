use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct List<T> {
    pub hint: String,
    pub list: VecDeque<T>,
}

impl<T> List<T> {
    pub fn push(&mut self, value: T) {
        self.list.push_back(value)
    }
    pub fn clear(&mut self) {
        self.list.clear()
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
        let list = VecDeque::from_iter(iter.into_iter().map(|v| O::from(v)));
        List { hint: "".to_string(), list }
    }
}

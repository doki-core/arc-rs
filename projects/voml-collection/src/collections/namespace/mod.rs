use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Namespace {
    pub path: Vec<String>,
}

impl Namespace {
    pub fn push<T>(&mut self, value: T)
    where
        String: From<T>,
    {
        self.path.push(String::from(value))
    }
    pub fn module(&self) -> &[String] {
        let len = self.path.len();
        if len != 0 { &self.path[0..len - 1] } else { &[] }
    }
    pub fn name(&self) -> &str {
        match self.path.last() {
            Some(s) => s.as_str(),
            None => "",
        }
    }
    pub fn clear(&mut self) {
        self.path.clear()
    }
}

impl<V> FromIterator<V> for Namespace
where
    String: From<V>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = V>,
    {
        let list = Vec::from_iter(iter.into_iter().map(|v| String::from(v)));
        Namespace { path: list }
    }
}

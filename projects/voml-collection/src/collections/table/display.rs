use std::fmt::{Debug, Formatter};

use super::*;

impl<T: Debug> Debug for Table<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let head = match self.hint.as_str() {
            "_" => "Table",
            s => s,
        };
        let mut w = &mut f.debug_struct(head);
        for (k, v) in self.list.iter().enumerate() {
            w = w.field(&k.to_string(), v)
        }
        for (k, v) in self.dict.iter() {
            w = w.field(k, v)
        }
        w.finish()
    }
}

impl<T> Table<T> {
    ///
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.list.is_empty() && self.dict.is_empty()
    }
    ///
    #[inline]
    pub fn is_list(&self) -> bool {
        self.dict.is_empty()
    }
    ///
    #[inline]
    pub fn is_dict(&self) -> bool {
        self.list.is_empty()
    }
}

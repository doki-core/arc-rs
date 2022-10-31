
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Table<T> {
    pub hint: String,
    pub list: Vec<T>,
    pub dict: IndexMap<String, T>,
}

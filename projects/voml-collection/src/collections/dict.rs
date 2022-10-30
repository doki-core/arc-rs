use indexmap::IndexMap;

#[derive(Clone, Debug, Eq)]
pub struct Dict<T> {
    pub hint: String,
    pub dict: IndexMap<String, T>,
}

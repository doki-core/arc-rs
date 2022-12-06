pub enum VQuery {
    Path(Vec<String>),
}

impl From<Vec<String>> for VQuery {
    fn from(value: Vec<String>) -> Self {
        VQuery::Path(value)
    }
}

impl From<&str> for VQuery {
    fn from(value: &str) -> Self {
        VQuery::Path(value.split('.').map(|s| s.to_string()).collect())
    }
}

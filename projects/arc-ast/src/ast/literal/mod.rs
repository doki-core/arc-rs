#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Symbol {
    Index(usize),
    Key(String),
}

impl Symbol {
    pub fn index(input: &str) -> Self {
        Self::Index(0)
    }
    pub fn key(input: impl Into<String>) -> Self {
        Self::Key(input.into())
    }
    pub fn boxed(self) -> Box<Symbol> {
        Box::new(self)
    }
}

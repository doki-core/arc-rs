use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Byte {
    handler: Option<String>,
    value: String,
}

impl Byte {
    pub fn get_handler(&self) -> Option<String> {
        self.handler.to_owned()
    }
}

use super::*;

impl Default for VonNode {
    fn default() -> Self {
        VonNode::Keyword("default")
    }
}

impl Default for Table {
    fn default() -> Self {
        Self { hint: "".to_string(), list: vec![], dict: Default::default() }
    }
}

impl Hash for Table {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hint.hash(state);
        for item in &self.list {
            item.hash(state)
        }
        for (k, v) in &self.dict {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl PartialEq for Table {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

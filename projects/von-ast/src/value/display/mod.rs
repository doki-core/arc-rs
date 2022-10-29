use std::fmt::{Debug, Display, Formatter};

use super::*;

pub struct PrettyPrint {}

impl Debug for VonNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VonNode::Keyword(v) => Debug::fmt(v, f),
            VonNode::Boolean(v) => Debug::fmt(v, f),
            VonNode::Number(v) => Debug::fmt(v, f),
            VonNode::Text(v) => Debug::fmt(v, f),
            VonNode::Table(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for VonNode {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Text {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Table {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// #[derive(Clone)]
// pub struct Text {
//     pub hint: Option<Identifier>,
//     pub value: String,
// }
//
// #[derive(Clone)]
// pub struct List {
//     pub hint: Option<Identifier>,
//     pub value: Vec<VonNode>,
// }
//
// #[derive(Clone)]
// pub struct Dict {
//     pub hint: Option<Identifier>,
//     pub value: IndexMap<String, VonNode>,
// }

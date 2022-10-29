use std::fmt::{Display, Formatter};

use super::*;

pub struct PrettyPrint {}

impl Display for VonNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

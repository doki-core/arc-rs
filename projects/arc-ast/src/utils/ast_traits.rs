use crate::ast::null;
use crate::AST;
use std::ops::{Index, Deref};

impl Index<usize> for AST {
    type Output = Self;

    fn index(&self, index: usize) -> &Self {
        match *self {
            AST::List(ref list) => list.get(index).unwrap_or(&null),
            _ => &null,
        }
    }
}

impl<'a> Index<&'a str> for AST {
    type Output = Self;

    fn index(&self, index: &str) -> &Self {
        match *self {
            AST::Dict(ref object) => &object[index],
            _ => &null,
        }
    }
}

impl Index<String> for AST {
    type Output = Self;
    fn index(&self, index: String) -> &Self {
        self.index(index.deref())
    }
}

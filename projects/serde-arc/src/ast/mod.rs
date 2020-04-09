pub mod ast_from;
pub mod ast_impl;
pub mod ast_traits;

use arc_number::Number;
use linked_hash_map::LinkedHashMap;
use std::collections::VecDeque;

#[derive(Clone, PartialEq)]
pub enum Arc {
    Null,
    Boolean(bool),
    Number(Number),
    Char(char),
    String(String),
    Cite(Path),
    List(VecDeque<Arc>),
    Dict(LinkedHashMap<String, Arc>),
    /// line with nothing or only whitespace
    EmptyLine,
    Key(KeyType, Path),
    HandlerString(Box<str>, String),
    HandlerNumber(Box<str>, Number),
    Comment(CommentType, String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommentType {
    /// one line comments
    /// % this is inline comment
    Inline,
    /// multiline comments
    /// %%%
    /// this is multiline comment
    /// %%%
    Block,
    /// type hint comment
    /// %
    /// %! this
    /// % this
    /// %!
    TypeHint,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyType {
    List,
    ListInherit,
    Dict,
    DictInherit,
}

pub type Path = Vec<KeyNode>;

#[derive(Debug, Clone, PartialEq)]
pub enum KeyNode {
    Key(Box<str>),
    Index(i32),
}

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
    Cite(KeyPath),
    List(VecDeque<Arc>),
    Dict(LinkedHashMap<Box<str>, Arc>),
    /// line with nothing or only whitespace
    EmptyLine,
    FreeDict(LinkedHashMap<KeyPath, Arc>),
    Record(KeyPath, Box<Arc>),
    Key(KeyType, KeyPath),
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyPath(pub Vec<KeyNode>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum KeyNode {
    Key(Box<str>),
    Index(i32),
}

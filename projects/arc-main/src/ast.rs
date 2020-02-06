use json::JsonValue;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::ops::{Deref, Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AST {
    Null,
    Boolean(bool),
    Number(Number),
    String(String),
    Cite(Vec<String>),
    List(Vec<AST>),
    Dict(HashMap<String, AST>),
    Macro,
}

#[allow(non_upper_case_globals)]
static null: AST = AST::Null;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Number {
    Integer(String),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Integer128(i128),
    Integer256(String),
    Unsigned(String),
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Unsigned128(u128),
    Unsigned256(String),
    Decimal(String),
    Decimal32(f32),
    Decimal64(f64),
}

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

impl AST {
    pub fn new_string(handler: &str, data: &str) -> AST {
        unimplemented!();
        AST::String(String::new())
    }
    pub fn new_number(handler: &str, data: &str) -> AST {
        unimplemented!();
    }
    pub fn new_dict() -> AST {
        unimplemented!();
        AST::Dict(HashMap::new())
    }
    pub fn new_list() -> AST {
        unimplemented!();
        AST::List(Vec::new())
    }
}

impl AST {
    pub fn is_string(&self) -> bool {
        match *self {
            AST::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            AST::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match *self {
            AST::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            AST::Null => true,
            _ => false,
        }
    }
}

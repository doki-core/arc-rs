use crate::ast::null;
use crate::Arc;
use serde::export::fmt::Error;
use serde::export::Formatter;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Deref, Index};

impl Display for Arc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match *self {
            Arc::Null => write!(f, "null"),
            Arc::Boolean(ref b) => write!(f, "{}", b),
            Arc::Cite(ref c) => write!(f, "${}", c.join(".")),
            Arc::Number(ref n) => write!(f, "{}", n),
            Arc::String(ref s) => write!(f, "{:?}", s),
            Arc::List(ref l) => {
                let mut code = vec![];
                let mut length = 0;
                for i in l {
                    let t = &format!("{}", i);
                    code.push(t.clone());
                    length += t.chars().count();
                }
                write!(f, "[{}]", code.join(", "))
            }
            Arc::Dict(ref map) => {
                let mut code = String::new();
                match map.len() {
                    0 => write!(f, "{{}}"),
                    1 => {
                        for (s, o) in map {
                            code.push_str(s);
                            code.push_str(":");
                            code.push_str(&format!("{}", o));
                        }
                        write!(f, "{{{}}}", code)
                    }
                    _ => {
                        for (s, o) in map {
                            code.push_str(s);
                            code.push_str(": ");
                            code.push_str(&format!("{}", o));
                            code.push_str(",\n    ");
                        }
                        write!(f, "{{\n    {}\n}}", code.trim_end())
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Index<usize> for Arc {
    type Output = Self;

    fn index(&self, index: usize) -> &Self {
        match *self {
            Arc::List(ref list) => list.get(index).unwrap_or(&null),
            _ => &null,
        }
    }
}

impl<'a> Index<&'a str> for Arc {
    type Output = Self;

    fn index(&self, index: &str) -> &Self {
        match *self {
            Arc::Dict(ref object) => &object[index],
            _ => &null,
        }
    }
}

impl Index<String> for Arc {
    type Output = Self;
    fn index(&self, index: String) -> &Self {
        self.index(index.deref())
    }
}

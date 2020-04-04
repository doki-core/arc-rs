use crate::Arc;

use std::{
    fmt::{self, Debug, Display, Error, Formatter},
    ops::{Deref, Index},
};

#[allow(unused_qualifications)]
impl Debug for Arc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Arc::Null => {
                let mut debug_trait_builder = f.debug_tuple("Null");
                debug_trait_builder.finish()
            }
            Arc::Boolean(v) => {
                let mut debug_trait_builder = f.debug_tuple("Boolean");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Number(v) => {
                let mut debug_trait_builder = f.debug_tuple("Number");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Char(v) => {
                let mut debug_trait_builder = f.debug_tuple("Char");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::String(v) => {
                let mut debug_trait_builder = f.debug_tuple("String");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Cite(v) => {
                let mut debug_trait_builder = f.debug_tuple("Cite");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::List(v) => {
                let mut debug_trait_builder = f.debug_tuple("List");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Dict(v) => {
                let mut debug_trait_builder = f.debug_tuple("Dict");
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
            Arc::Macro(s, v) => {
                let mut debug_trait_builder = f.debug_tuple("Macro");
                let _ = debug_trait_builder.field(s);
                let _ = debug_trait_builder.field(v);
                debug_trait_builder.finish()
            }
        }
    }
}

impl Display for Arc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

// impl Index<usize> for Arc {
// type Output = Self;
//
// fn index(&self, index: usize) -> &Self {
// match *self {
// Arc::List(ref list) => list.get(index).unwrap_or(&Arc::Null),
// _ => &Arc::Null,
// }
// }
// }
impl Index<isize> for Arc {
    type Output = Self;

    fn index(&self, index: isize) -> &Self {
        match *self {
            Arc::List(ref list) => {
                if index >= 0 {
                    list.get(index as usize).unwrap_or(&Arc::Null)
                }
                else {
                    let i = list.len() as isize + index;
                    list.get(i as usize).unwrap_or(&Arc::Null)
                }
            }
            _ => &Arc::Null,
        }
    }
}

impl<'a> Index<&'a str> for Arc {
    type Output = Self;

    fn index(&self, index: &str) -> &Self {
        match *self {
            Arc::Dict(ref object) => &object[index],
            _ => &Arc::Null,
        }
    }
}

impl Index<String> for Arc {
    type Output = Self;
    fn index(&self, index: String) -> &Self {
        self.index(index.deref())
    }
}

impl PartialEq<bool> for Arc {
    fn eq(&self, other: &bool) -> bool {
        match self {
            Arc::Boolean(arc) => arc == other,
            _ => false,
        }
    }
}

impl PartialEq<str> for Arc {
    fn eq(&self, other: &str) -> bool {
        match self {
            Arc::String(arc) => arc.as_str() == other,
            _ => false,
        }
    }
}

impl PartialEq<String> for Arc {
    fn eq(&self, other: &String) -> bool {
        match self {
            Arc::String(arc) => arc == other,
            _ => false,
        }
    }
}

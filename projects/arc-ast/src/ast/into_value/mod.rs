use crate::{
    ast::{ASTKind, AST},
    value::{Dict, Integer, List},
    Value,
};
use std::ops::Neg;

impl From<AST> for Value {
    fn from(ast: AST) -> Self {
        Value::from(ast.kind)
    }
}

impl From<ASTKind> for Value {
    fn from(ast: ASTKind) -> Self {
        let mut builder = Scope::default();
        builder.build(ast)
    }
}

pub struct Scope {
    top: Value,
    pin_path: Vec<Vec<Value>>,
    key_path: Vec<Vec<Value>>,
}

impl<'a> Default for Scope {
    fn default() -> Self {
        Self { top: Value::from(Dict::default()), pin_path: vec![], key_path: vec![] }
    }
}

impl Scope {
    pub fn build(&mut self, ast: ASTKind) -> Value {
        match ast {
            ASTKind::Program(v) | ASTKind::Dict(v) => v.into_iter().for_each(|item| self.visit_ast(item.kind)),
            ASTKind::String(v) => self.top = Value::from(*v),
            ASTKind::Integer(v) => self.top = Value::from(*v),
            _ => unimplemented!("ASTKind::{:?}", ast),
        }
        self.top.to_owned()
    }

    pub fn visit_ast(&mut self, ast: ASTKind) {
        match ast {
            ASTKind::ListScope(depth, path) | ASTKind::DictScope(depth, path) => {
                // println!("{} vs {}", depth,self.pin_path.len());
                match depth >= self.pin_path.len() {
                    true => self.push_pin(path.kind),
                    false => {
                        self.pin_path.truncate(depth);
                        self.push_pin(path.kind)
                    }
                }
                self.get_pointer();
            }
            ASTKind::List(v) => {
                for (index, item) in v.into_iter().enumerate() {
                    self.push_index(index);
                    self.visit_ast(item.kind);
                    self.pop_index();
                }
            }
            ASTKind::Dict(v) => {
                for item in v {
                    self.visit_ast(item.kind);
                }
            }
            ASTKind::Pair(key, value) => {
                self.push_key(key.kind);
                self.visit_ast(value.kind);
                self.pop_key();
            }
            ASTKind::Null => {
                self.get_pointer();
            }
            ASTKind::Cite(v) => {
                let cite = self.extract_namespace(v.kind);
                *self.get_pointer() = self.top.get_value(&cite).clone();
            }
            ASTKind::Boolean(v) => *self.get_pointer() = Value::Boolean(v),
            ASTKind::Integer(v) => *self.get_pointer() = Value::Integer(v),
            ASTKind::Decimal(v) => *self.get_pointer() = Value::Decimal(v),
            ASTKind::String(v) => *self.get_pointer() = Value::String(v),
            _ => unimplemented!("ASTKind::{:?}", ast),
        }
    }

    fn get_pointer(&mut self) -> &mut Value {
        let mut pointer = &mut self.top;
        for path in self.pin_path.iter().flatten().chain(self.key_path.iter().flatten()) {
            match path {
                Value::String(key) => pointer = pointer.ensure_key(key.as_str().to_string()),
                Value::Integer(index) => {
                    pointer = pointer.ensure_index(index);
                }
                _ => unreachable!(),
            }
        }
        return pointer;
    }

    fn push_pin(&mut self, namespace: ASTKind) {
        let namespace = self.extract_namespace(namespace);
        self.pin_path.push(namespace)
    }

    fn push_key(&mut self, namespace: ASTKind) {
        let namespace = self.extract_namespace(namespace);
        self.key_path.push(namespace)
    }

    fn pop_key(&mut self) -> Option<Vec<Value>> {
        self.key_path.pop()
    }

    fn push_index(&mut self, index: usize) {
        let namespace = vec![Value::from(index)];
        self.key_path.push(namespace)
    }

    fn pop_index(&mut self) -> Option<Vec<Value>> {
        self.key_path.pop()
    }

    fn extract_namespace(&self, namespace: ASTKind) -> Vec<Value> {
        let mut out = vec![];
        match namespace {
            ASTKind::Namespace(ns) => {
                for item in ns {
                    match item.kind {
                        ASTKind::String(v) => out.push(Value::from(*v)),
                        ASTKind::Integer(v) => out.push(Value::from(*v)),
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        };
        return out;
    }
}

impl Value {
    pub fn get_value(&self, path: &[Value]) -> &Value {
        unimplemented!()
    }

    pub fn ensure_key(&mut self, key: String) -> &'_ mut Value {
        match self {
            Value::Null => {
                *self = Dict::empty();
                self.ensure_key(key)
            }
            Value::Dict(dict) => dict.ensure_key(key),
            _ => unimplemented!("{:?}", self),
        }
    }
    pub fn ensure_index(&mut self, index: &Integer) -> &'_ mut Value {
        match self {
            Value::Null => {
                *self = List::empty();
                self.ensure_index(index)
            }
            Value::List(list) => match index.get_index() {
                Some(i_index) => {
                    let u_index = match i_index >= 0 {
                        true => i_index as usize,
                        false => list.length() - i_index.neg() as usize,
                    };
                    list.ensure_index(u_index)
                }
                None => unreachable!(),
            },
            _ => unimplemented!("{:?}", self),
        }
    }
}

impl Dict {
    pub fn ensure_key(&mut self, key: String) -> &'_ mut Value {
        self.entry(key).or_default()
    }
}

impl List {
    pub fn ensure_index(&mut self, index: usize) -> &'_ mut Value {
        self.entry(index).or_default()
    }
}

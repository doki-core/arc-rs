use crate::{ast::AST, Value};
use crate::ast::ASTKind;
use crate::value::Dict;
use indexmap::IndexMap;

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
        Self {
            top: Value::from(Dict::default()),
            pin_path: vec![],
            key_path: vec![]
        }
    }
}


impl Scope {
    pub fn build(&mut self, ast: ASTKind) -> Value {
        match ast {
            ASTKind::Program(_) => {
                unimplemented!()
            }
            ASTKind::Dict(ast) => {
                for item in ast {
                    match item.kind {
                        ASTKind::Pair(key, value) => {
                            *self.get_pointer(*key) = self.get_item(value.kind)
                        }
                        _ => unimplemented!("ASTKind::{:#?} => {{}}", item.kind)
                    }
                }
            }

            _ => unimplemented!("ASTKind::{:?}", ast)
        }
        return self.top.to_owned();
    }
    fn get_item(&mut self, ast: ASTKind) ->Value {
        match ast {
            ASTKind::Dict(v) => {
                let dict = IndexMap::<String,Value>::new();
                for item in v {
                    match item.kind {
                        ASTKind::Pair(key, value) => {
                            *self.get_pointer(key.kind) = self.get_item(value.kind)
                        }
                        _ => unimplemented!("ASTKind::{:#?} => {{}}", item.kind)
                    }
                }
                Value::from(dict)
            }
            ASTKind::String(v) => {
                Value::from(*v)
            }
            _ => unimplemented!("ASTKind::{:?}", ast)
        }
    }

    fn get_pointer(&mut self, namespace: ASTKind) -> &mut Value {
        let namespace = self.extract_namespace(namespace);
        let mut pointer = &mut self.top;
        for path in self.pin_path.iter().flatten().chain(self.key_path.iter().flatten()).chain(namespace.iter()) {
            match path {
                Value::String(key) => {
                    pointer = pointer.ensure_key(key.as_str().to_string())
                }
                Value::Number(index) => {
                    println!("{:?}", index);
                    unimplemented!()
                }
                _ => unreachable!()
            }
        }
        return pointer
    }

    fn extract_namespace(&self, namespace: ASTKind )->Vec<Value> {
        let mut out = vec![];
        match namespace {
            ASTKind::Namespace(ns) => {
                for item in ns {
                    match item.kind {
                        ASTKind::String(v) => {
                            out.push(Value::from(*v))
                        }
                        ASTKind::Number(v) => {  out.push(Value::from(*v))},
                        _ => unreachable!()
                    }

                }
            },
            _ => unreachable!()
        };
        return out
    }
}

impl Value {
    pub fn ensure_key(&mut self, key: String) -> &'_ mut Value {
        match self {
            Value::Dict(dict) => {dict.ensure_key(key)}
            _ => unimplemented!("{:?}",self)
        }
    }
}

impl Dict  {
    pub fn ensure_key(&mut self, key: String) -> &'_ mut Value {
        self.entry(key).or_default()
    }
}
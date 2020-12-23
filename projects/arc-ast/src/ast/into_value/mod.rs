use crate::{ast::AST, Value};
use crate::ast::ASTKind;
use crate::value::Dict;

impl From<AST> for Value {
    fn from(ast: AST) -> Self {
        Value::from(ast.kind)
    }
}


impl From<ASTKind> for Value {
    fn from(ast: ASTKind) -> Self {
        match ast {
            ASTKind::Program(ast) => {
                let mut builder = Scope::default();
                builder.build(ast)
            }
            ASTKind::Dict(ast) => {
                let mut builder = Scope::default();
                builder.build(ast)
            }
            _ => unimplemented!("{:?}", ast)
        }
    }
}


pub struct Scope {
    top: Value,
    pin_path: Vec<Vec<Value>>,
}

impl<'a> Default for Scope {
    fn default() -> Self {
        Self {
            top: Value::from(Dict::default()),
            pin_path: vec![],
        }
    }
}


impl Scope {
    pub fn build(&mut self, ast: Vec<AST>) -> Value {
        for item in ast {
            match item.kind {
                ASTKind::Pair(key, value) => {
                    self.get_pointer(*key);
                    unimplemented!()
                }
                _ => unimplemented!("ASTKind::{:#?} => {{}}", item.kind)
            }
        }
        return self.top.to_owned();
    }
    pub fn get_pointer(&mut self, namespace: AST) -> &mut Value {
        let namespace = self.extract_namespace(namespace);
        for path in self.pin_path.iter().flatten().chain(namespace.iter()) {
            match path {
                Value::String(key) => {
                    println!("{:?}", key)
                }
                Value::Number(index) => {
                    println!("{:?}", index)
                }
                _ => unreachable!()
            }
        }
        unimplemented!()
    }
    pub fn extract_namespace(&self, namespace: AST )->Vec<Value> {
        let mut out = vec![];
        match namespace.kind {
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
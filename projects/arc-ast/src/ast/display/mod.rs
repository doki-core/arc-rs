use std::fmt::{self, Debug, Formatter, Display};
use crate::AST;
use crate::ast::ASTKind;

impl Debug for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST { kind, range, additional } => {
                let mut builder = f.debug_struct("AST");
                builder.field("kind", kind);
                builder.field("range", range);
                if let Some(s) = additional {
                    builder.field("additional", s);
                }
                builder.finish()
            }
        }
    }
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ASTKind::Namespace(names) => {
                for item in names {
                    match &item.kind {
                        ASTKind::String(s) => {
                            write!(f, "{}", s.value)?
                        }
                        ASTKind::Integer(n) => {
                            write!(f, "{}", n.value)?
                        },
                        _ => return Ok(())
                    }
                }
                Ok(())
            }
            _ => Debug::fmt(self, f)
        }
    }
}
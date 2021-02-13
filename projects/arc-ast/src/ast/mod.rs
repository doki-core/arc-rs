mod into_value;
mod literal;
mod range;

pub use crate::ast::range::TextRange;
use crate::{
    value::{parse_number, Decimal, Integer, Text, TextDelimiter},
    Value,
};
use num::{BigInt, Num};
use std::fmt::{self, Debug, Formatter};

// use bigdecimal::BigDecimal;
// use num::BigInt;

#[derive(Clone, Eq, PartialEq)]
pub struct AST {
    pub kind: ASTKind,
    pub range: Option<TextRange>,
    pub additional: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind {
    /// Placeholder
    /// Unreachable structure
    None,
    /// Root of all nodes
    Program(Vec<AST>),
    /// Flattened structure
    Sequence(Vec<AST>),
    /// Whitespace, Newline, Comment
    Span(String),
    /// `[list.scope]`
    ListScope(usize, Box<AST>),
    /// `{dict.scope}`
    DictScope(usize, Box<AST>),
    ///
    Pair(Box<AST>, Box<AST>),
    /// `null`
    Null,
    /// `true` | `false`
    Boolean(bool),
    String(Box<Text>),
    Namespace(Vec<AST>),
    Integer(Box<Integer>),
    Decimal(Box<Decimal>),
    Cite(Box<AST>),
    Dict(Vec<AST>),
    List(Vec<AST>),
}

impl Debug for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST { kind, range, additional } => {
                let mut builder = f.debug_struct("AST");
                builder.field("kind", kind);
                if let Some(s) = range {
                    builder.field("range", s);
                }
                if let Some(s) = additional {
                    builder.field("additional", s);
                }
                builder.finish()
            }
        }
    }
}

impl Default for AST {
    fn default() -> Self {
        Self { kind: ASTKind::None, range: Default::default(), additional: None }
    }
}

impl From<ASTKind> for AST {
    fn from(kind: ASTKind) -> Self {
        Self { kind, range: None, additional: None }
    }
}

// impl AST {
//     pub fn as_vec(&self) -> Vec<AST> {
//         match &self.kind {
//             ASTKind::Program(v) | ASTKind::Block(v) => v.to_owned(),
//             _ => vec![],
//         }
//     }
// }

impl AST {
    pub fn set_additional(&mut self, info: impl Into<String>) {
        self.additional = Some(info.into())
    }
    pub fn set_range(&mut self, range: TextRange) {
        self.range = Some(range)
    }
}

impl AST {
    pub fn program(children: Vec<AST>) -> Self {
        Self { kind: ASTKind::Program(children), range: None, additional: None }
    }
    pub fn block(children: Vec<AST>) -> Self {
        Self { kind: ASTKind::Sequence(children), range: None, additional: None }
    }
    // pub fn statement(children: Vec<AST>, r: TextRange) -> Self {
    //     Self { kind: ASTKind::Statement(children), range: box_range(r) }
    // }
    //
    // pub fn if_else_chain(cds: Vec<AST>, acts: Vec<AST>, r: TextRange) -> Self {
    //     let kind = ASTKind::IfElseChain(Box::new(IfElseChain::build(cds, acts)));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn for_in_loop(pattern: AST, terms: AST, block: AST, guard: Option<AST>, for_else: Option<AST>, r: TextRange) -> Self {
    //     let kind = ASTKind::ForInLoop(Box::new(ForInLoop { pattern, terms, guard, block, for_else }));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn expression(children: AST, eos: bool, r: TextRange) -> Self {
    //     let kind = ASTKind::Expression(Box::new(children), eos);
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn operation(op: &str, kind: &str, r: TextRange) -> Self {
    //     let o = match kind {
    //         "<" => Operator::prefix(op),
    //         ">" => Operator::suffix(op),
    //         _ => Operator::infix(op),
    //     };
    //     let kind = ASTKind::Operator(Box::new(o));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn string_expression(value: Vec<AST>, handler: Option<AST>, r: TextRange) -> Self {
    //     let kind = ASTKind::StringExpression(Box::new(StringExpression { handler, inner: value }));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn infix_expression(op: AST, lhs: AST, rhs: AST, r: TextRange) -> Self {
    //     let kind = ASTKind::InfixExpression(Box::new(InfixExpression { op, lhs, rhs }));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn prefix_expression(op: AST, rhs: AST, r: TextRange) -> Self {
    //     let kind = ASTKind::PrefixExpression(Box::new(UnaryExpression { op, base: rhs }));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn suffix_expression(op: AST, lhs: AST, r: TextRange) -> Self {
    //     let kind = ASTKind::PrefixExpression(Box::new(UnaryExpression { op, base: lhs }));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn call_chain(chain: CallChain, r: TextRange) -> Self {
    //     Self { kind: ASTKind::CallChain(Box::new(chain)), range: box_range(r) }
    // }
    //
    // pub fn call_index(index: &str, r: TextRange) -> Self {
    //     let n = BigInt::parse_bytes(index.as_bytes(), 10).unwrap_or_default();
    //     Self { kind: ASTKind::CallIndex(Box::new(n)), range: box_range(r) }
    // }
    //
    // pub fn template(value: Template, r: TextRange) -> Self {
    //     Self { kind: ASTKind::Template(Box::new(value)), range: box_range(r) }
    // }
    //
    pub fn null() -> Self {
        Self { kind: ASTKind::Null, range: None, additional: None }
    }

    pub fn boolean(value: bool) -> Self {
        Self { kind: ASTKind::Boolean(value), range: None, additional: None }
    }
    pub fn string(value: Text) -> Self {
        Self { kind: ASTKind::String(Box::new(value)), range: None, additional: None }
    }
    // pub fn string_escaped(value: String, r: TextRange) -> Self {
    //     Self { kind: ASTKind::EscapedText(value), range: box_range(r) }
    // }
    pub fn integer(value: &str) -> Self {
        let n = BigInt::from_str_radix(value, 10).unwrap_or_default();
        Self { kind: ASTKind::Integer(Box::new(Integer::from(n))), range: None, additional: None }
    }
    pub fn number(value: &str) -> Self {
        let kind = match parse_number(value) {
            Some(Value::Integer(n)) => ASTKind::Integer(n),
            Some(Value::Decimal(n)) => ASTKind::Decimal(n),
            _ => ASTKind::Null,
        };
        Self { kind, range: None, additional: None }
    }
    pub fn namespace(value: Vec<AST>) -> Self {
        Self { kind: ASTKind::Namespace(value), range: None, additional: None }
    }
    pub fn list(value: Vec<AST>) -> Self {
        Self { kind: ASTKind::List(value), range: None, additional: None }
    }
    pub fn dict(pairs: Vec<AST>) -> Self {
        Self { kind: ASTKind::Dict(pairs), range: None, additional: None }
    }
    pub fn pair(lhs: AST, rhs: AST) -> Self {
        Self { kind: ASTKind::Pair(Box::new(lhs), Box::new(rhs)), range: None, additional: None }
    }
}

impl Text {
    pub fn string_escaped(value: impl Into<String>, handler: impl Into<String>, delimiter: usize) -> Self {
        let handler = handler.into();
        let handler = match handler.len() {
            0 => None,
            _ => Some(handler),
        };
        Text { handler, delimiter: TextDelimiter::Quotation(delimiter), value: value.into() }
    }
    pub fn string_literal(value: impl Into<String>, handler: impl Into<String>, delimiter: usize) -> Self {
        let handler = handler.into();
        let handler = match handler.len() {
            0 => None,
            _ => Some(handler),
        };
        Text { handler, delimiter: TextDelimiter::Apostrophe(delimiter), value: value.into() }
    }
    pub fn string_bare(value: impl Into<String>) -> Self {
        Text { handler: None, delimiter: TextDelimiter::Bare, value: value.into() }
    }
}

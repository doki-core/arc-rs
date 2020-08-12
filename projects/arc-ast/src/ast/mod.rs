mod range;

use std::fmt::{self, Debug, Display, Formatter};

pub use crate::ast::range::TextRange;
use bigdecimal::BigDecimal;
use num::BigInt;

#[derive(Clone, Eq, PartialEq)]
pub struct AST {
    pub kind: ASTKind,
    pub range: Option<Box<TextRange>>,
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
    ///
    Pair(Box<AST>, Box<AST>),

    Null,
    Boolean(bool),
    // String(Box<ASTString>),
    // Symbol(Box<Symbol>),
    // Integer(Box<ASTInteger>),
    // Decimal(Box<ASTDecimal>),
}

impl Debug for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST { kind, range } => {
                let mut builder = f.debug_struct("AST");
                builder.field("kind", kind);
                match range {
                    None => (),
                    Some(s) => {
                        builder.field("range", s.as_ref());
                    }
                }
                builder.finish()
            }
        }
    }
}


impl Default for AST {
    fn default() -> Self {
        Self { kind: ASTKind::None, range: Default::default() }
    }
}

impl From<ASTKind> for AST {
    fn from(kind: ASTKind) -> Self {
        Self { kind, range: None }
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
    pub fn program(children: Vec<AST>) -> Self {
        Self { kind: ASTKind::Program(children), range: Default::default() }
    }
    pub fn block(children: Vec<AST>, r: TextRange) -> Self {
        Self { kind: ASTKind::Sequence(children), range: r.boxed() }
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
    // pub fn list(value: Vec<AST>, r: TextRange) -> Self {
    //     Self { kind: ASTKind::List(value), range: box_range(r) }
    // }
    //
    // pub fn null(r: TextRange) -> Self {
    //     Self { kind: ASTKind::Null, range: box_range(r) }
    // }
    //
    // pub fn boolean(value: bool, r: TextRange) -> Self {
    //     Self { kind: ASTKind::Boolean(value), range: box_range(r) }
    // }
    // pub fn string(value: String, r: TextRange) -> Self {
    //     Self { kind: ASTKind::UnescapedText(value), range: box_range(r) }
    // }
    // pub fn string_escaped(value: String, r: TextRange) -> Self {
    //     Self { kind: ASTKind::EscapedText(value), range: box_range(r) }
    // }
    // pub fn integer(value: &str, base: u32, r: TextRange) -> Self {
    //     let n = BigInt::parse_bytes(value.as_bytes(), base).unwrap_or_default();
    //     Self { kind: ASTKind::Integer(Box::new(n)), range: box_range(r) }
    // }
    // pub fn decimal(value: &str, base: u32, r: TextRange) -> Self {
    //     let n = BigDecimal::parse_bytes(value.as_bytes(), base).unwrap_or_default();
    //     Self { kind: ASTKind::Decimal(Box::new(n)), range: box_range(r) }
    // }
    // pub fn symbol(value: Vec<AST>, r: TextRange) -> Self {
    //     Self { kind: ASTKind::Symbol(Box::new(Symbol::from(value))), range: box_range(r) }
    // }
}


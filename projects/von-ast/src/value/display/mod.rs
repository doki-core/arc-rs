use std::fmt::{Arguments, Debug, Display, Formatter, Write};

use super::*;

pub struct PrettyPrint {
    buffer: String,
}

impl Debug for VonNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VonNode::Keyword(v) => Debug::fmt(v, f),
            VonNode::Boolean(v) => Debug::fmt(v, f),
            VonNode::Number(v) => Debug::fmt(v, f),
            VonNode::Text(v) => Debug::fmt(v, f),
            VonNode::Table(v) => Debug::fmt(v, f),
        }
    }
}

impl Write for PrettyPrint {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.buffer.write_char(c)
    }
    fn write_fmt(&mut self, args: Arguments<'_>) -> std::fmt::Result {
        self.buffer.write_fmt(args)
    }
}

impl Display for VonNode {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Text {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Table {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PrettyPrint {}

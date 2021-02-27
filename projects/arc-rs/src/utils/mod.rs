pub use arc_ast::utils::*;

use crate::{ParserConfig, Value};
use crate::Result;

pub fn parse_arc(text: &str) -> Result<Value> {
    let cfg = ParserConfig::default();
    Ok(Value::from(cfg.parse(text)?))
}




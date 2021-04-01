use super::*;
use std::path::PathBuf;
use lsp_types::{Range, Position};

///
pub struct ParserConfig {
    ///
    pub tab_size: usize,
    ///
    pub file_path: Option<PathBuf>,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self { tab_size: 4, file_path: None }
    }
}

impl ParserConfig {
    ///
    pub fn get_position(&self, s: Span) -> Range {
        let us = s.start_pos().line_col();
        let es = s.end_pos().line_col();
        Range {
            // index: s.start_pos().pos() as u64,
            start: Position {
                line: us.0 as u64,
                character: us.1 as u64,
            },
            end: Position {
                line: es.0 as u64,
                character: es.1 as u64,
            },
        }
    }
}

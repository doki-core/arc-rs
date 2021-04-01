use super::*;
use std::path::PathBuf;

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
    pub fn get_position(&self, s: Span) -> TextRange {
        let us = s.start_pos().line_col();
        let es = s.end_pos().line_col();
        TextRange {
            // index: s.start_pos().pos() as u64,
            start: (us.0 as u64, us.1 as u64),
            end: (es.0 as u64, es.1 as u64),
        }
    }
}

use crate::Settings;

impl Default for Settings {
    fn default() -> Self {
        Settings { symbol_set: String::from(":"), pest_indent: 0, pest_sequence_first: false }
    }
}
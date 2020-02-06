use crate::Settings;
pub use textwrap::indent;

impl Default for Settings {
    fn default() -> Self {
        Settings {
            arc_symbol_set: String::from("="),
            arc_indent: 4,
            arc_dict_separator: String::from(""),
            arc_list_separator: String::from(","),
            arc_list_max_length: 128,
        }
    }
}
pub use self::{
    parser::parse,
    value::{display::PrettyPrint, *},
};
pub use voml_collection::{Number, Text};

pub type Table = voml_collection::Table<VonNode>;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod parser;
mod value;

use crate::utils::arc_traits::ToArc;
use std::{
    fs::{read_to_string, File},
    io::Write,
};
use toml_edit::{Document, Item};

pub fn file_to_arc(path_from: &str, path_to: &str) -> Result<(), std::io::Error> {
    let r = read_to_string(path_from)?;
    let s = to_arc(&r).unwrap_or_default();
    let mut file = File::create(path_to)?;
    file.write_all(s.as_bytes())?;
    return Ok(());
}

pub fn to_arc(text: &str) -> Result<String, &'static str> {
    let toml = text.parse::<Document>().expect("invalid doc");
    let out = match &toml.root {
        Item::None => String::from("toml = null"),
        Item::Value(v) => format!("toml = {}", v),
        Item::Table(table) => {
            let mut pairs = vec![];
            for (k, v) in table.iter() {
                pairs.push(format!("{} = {}", k, v.to_arc()))
            }
            pairs.join("\n")
        }
        Item::ArrayOfTables(array) => array.to_arc(),
    };
    Ok(out)
}

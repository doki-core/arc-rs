use yaml_rust::{YamlLoader, Yaml};
use yaml_rust::yaml::{Hash, Array};
use crate::ToArc;
use std::fs::{read_to_string, File};
use std::io::Write;
use textwrap::indent;


pub fn file_to_arc(path_from: &str, path_to: &str) -> Result<(), std::io::Error> {
    let r = read_to_string(path_from)?;
    let s = to_arc(&r).unwrap_or_default();
    let mut file = File::create(path_to)?;
    file.write_all(s.as_bytes())?;
    return Ok(());
}

pub fn to_arc(text: &str) -> Result<String, &'static str> {
    let docs = YamlLoader::load_from_str(text).expect("ParseError");
    if docs.len() == 1 {
        match docs[0].clone() {
            Yaml::Hash(ref h) => {
                Ok(parse_pairs(h).join("\n"))
            }
            Yaml::Array(ref a) => {
                Ok(parse_list(a))
            }
            _ => Ok(format!("yaml = {}", docs.to_arc()))
        }
    } else {
        Ok(parse_list(&docs))
    }
}


pub fn parse_list(a: &Array) -> String {
    let mut code = String::from("[yaml]\n");
    for i in a {
        match i {
            Yaml::Hash(h) => {
                let s = indent(&parse_pairs(h).join("\n"), "  ");
                code.push_str(&format!("* {}", s.trim_start()))
            }
            _ => code.push_str(&format!("> {}\n", i.to_arc()))
        }
    }
    return code;
}

pub fn parse_pairs(h: &Hash) -> Vec<String> {
    let mut kv = vec![];
    for (k, v) in h {
        let key = parse_key(k);
        kv.push(format!("{} = {}", key, v.to_arc()))
    }
    return kv;
}

pub fn parse_key(k: &Yaml) -> String {
    match k {
        Yaml::String(r) => {
            let s = format!("{:?}", r);
            if s.contains(".") || s.contains(" ") {
                s
            } else {
                s.trim_matches('"').to_string()
            }
        }
        Yaml::Array(a) => {
            let mut code = vec![];
            for i in a {
                code.push(i.to_arc())
            }
            code.join(".")
        }
        _ => k.to_arc()
    }
}


use yaml_rust::{YamlLoader, Yaml};
use yaml_rust::yaml::{Hash, Array};
use textwrap::{indent};
use crate::ToArc;

pub fn file_to_arc() -> Result<String, &'static str> {
    unimplemented!()
}


pub fn to_arc(text: &str) -> Result<String, &'static str> {
    let docs = YamlLoader::load_from_str(text).expect("ParseError");
    if docs.len() == 1 {
        match docs[0].clone() {
            Yaml::Hash(ref h) => {
                Ok(parse_pairs(h).join("\n"))
            }
            Yaml::Array(ref a) => {
                Ok(format!("yaml = {}", docs[0].to_arc()))
            }
            _ => Ok(format!("yaml = {}", docs.to_arc()))
        }
    } else {
        Ok(format!("yaml = {}", docs.to_arc()))
    }
}



pub fn parse_pairs(h: &Hash) -> Vec<String> {
    let mut kv = vec![];
    for (k, v) in h {
        let s = k.to_arc();
        let key = if s.contains(".") || s.contains(" ") {
            s
        }
        else {
            s.trim_matches('"').to_string()
        };
        kv.push(format!("{} = {}", key, v.to_arc()))
    }
    return kv;
}



#[test]
fn main() {
    let s =
        r#"
? - Detroit Tigers
  - Chicago cubs
:
  - 2001-07-23

? [ New York Yankees,
    Atlanta Braves ]
: [ 2001-07-02, 2001-08-12,
    2001-08-14 ]
"#;
    let t = to_arc(s);
    println!("{}", t.unwrap_or_default());
    unreachable!()
}
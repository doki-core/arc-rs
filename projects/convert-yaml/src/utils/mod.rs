use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use std::collections::hash_map::RandomState;
use yaml_rust::yaml::{Hash, Array};
use textwrap::{indent,dedent};

trait ToArc {
    fn to_arc(&self) -> String;
}

impl ToArc for Yaml {
    fn to_arc(&self) -> String {
        match self {
            Yaml::Real(r) => {
                if r.starts_with('.') {
                    format!("0{}", r)
                } else if r.ends_with('.') {
                    format!("{}0", r)
                } else {
                    format!("{}", r)
                }
            }
            Yaml::Integer(i) => {
                format!("{}", i)
            }
            Yaml::String(s) => {
                format!("{:?}", s)
            }
            Yaml::Boolean(b) => {
                println!("{:#?}", b);
                unreachable!()
            }
            Yaml::Array(a) => {
                a.to_arc()
            }
            Yaml::Hash(h) => {
                h.to_arc()
            }
            Yaml::Alias(a) => {
                println!("{:#?}", a);
                unreachable!()
            }
            Yaml::Null => {
                String::from("null")
            }
            Yaml::BadValue => {
                String::from("null")
            }
        }
    }
}

impl ToArc for Hash {
    fn to_arc(&self) -> String {
        let mut kv = String::new();
        for (k, v) in self {
            kv.push_str(&format!("{} = {}\n", k.to_arc(), v.to_arc()))
        }
        return format!("{{\n{}}}", indent(&kv, "    "));
    }
}

impl ToArc for Array {
    fn to_arc(&self) -> String {
        let mut max = 0;
        let mut len = 0;
        let mut v = vec![];
        for a in self {
            let s = a.to_arc();
            let l = s.lines().count();
            if l > max {
                max = l
            }
            len += s.len();//for fast
            v.push(s)
        }
        if len < 128 && max <= 1 {
            format!("[{}]", v.join(", "))
        } else {
            format!("[{}]", indent(&v.join("\n"), "    "))
        }
    }
}

fn file_to_arc() -> Result<String, &'static str> {
    unimplemented!()
}


fn to_arc(text: &str) -> Result<String, &'static str> {
    let docs = YamlLoader::load_from_str(text).expect("ParseError");
    if docs.len() == 1 {
        match docs[0].clone() {
            Yaml::Hash(i) => {
                let mut kv = String::new();
                for (k, v) in i {
                    kv.push_str(&format!("{} = {}\n", k.to_arc(), v.to_arc()))
                }
                Ok(kv)
            }
            Yaml::Array(a) => {
                Ok(docs[0].to_arc())
            }
            _ => Ok(docs.to_arc())
        }
    } else {
        Ok(docs.to_arc())
    }
}

#[test]
fn main() {
    let s =
        r#"
american:
  - Boston Red Sox
  - Detroit Tigers
  - New York Yankees
national:
  - New York Mets
  - Chicago Cubs
  - Atlanta Braves
"#;
    let t = to_arc(s);
    println!("{}", t.unwrap_or_default());
    unreachable!()
}
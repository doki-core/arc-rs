use yaml_rust::{ Yaml};
use crate::utils::parse_pairs;
use yaml_rust::yaml::{Hash, Array};
use textwrap::{indent, };

pub trait ToArc {
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
        let kv = parse_pairs(self);
        if kv.len() == 1 {
            format!("{{{}}}", kv[0])
        } else {
            format!("{{\n{}}}", indent(&kv.join("\n"), "    "))
        }
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
            format!("[\n{}]", indent(&v.join("\n"), "    "))
        }
    }
}
use crate::Value;
use yaml_rust::{
    yaml::{Array, Hash},
    Yaml,
};

pub trait ToArc {
    fn to_arc(&self) -> String;
}

impl From<Yaml> for Value {
    fn from(yaml: Yaml) -> Self {
        match yaml {
            Yaml::Real(r) => {
                if r.to_lowercase().contains('e') {
                    match r.parse::<f64>() {
                        Ok(o) => format!("{}", o),
                        Err(_) => String::from("null"),
                    }
                }
                else if r.to_lowercase().contains("n") {
                    match r.to_lowercase().as_str() {
                        ".inf" | "+.inf" | "inf" => format!("{:#X}f64", f64::INFINITY.to_bits()),
                        "-inf" | "-.inf" => format!("{:#X}f64", f64::NEG_INFINITY.to_bits()),
                        ".nan" | "nan" => format!("{:#X}f64", f64::NAN.to_bits()),
                        _ => String::from("null"),
                    }
                }
                else if r.starts_with('.') {
                    format!("0{}", r)
                }
                else if r.ends_with('.') {
                    format!("{}0", r)
                }
                else {
                    format!("{}", r)
                }
            }
            Yaml::Integer(i) => format!("{}", i),
            Yaml::String(s) => format!("{:?}", s),
            Yaml::Boolean(b) => format!("{}", b),
            Yaml::Array(a) => a.to_arc(),
            Yaml::Hash(h) => h.to_arc(),
            Yaml::Alias(a) => {
                println!("{:#?}", a);
                unreachable!()
            }
            Yaml::Null => String::from("null"),
            Yaml::BadValue => String::from("null"),
        }
    }
}

impl From<Hash> for Value {
    fn from(_: Hash) -> Self {
        let kv = parse_pairs(self);
        if kv.len() == 1 { format!("{{{}}}", kv[0]) } else { format!("{{\n{}}}", indent(&kv.join("\n"), "    ")) }
    }
}

impl From<Array> for Value {
    fn from(_: Array) -> Self {
        let mut max = 0;
        let mut len = 0;
        let mut v = vec![];
        for a in self {
            let s = a.to_arc();
            let l = s.lines().count();
            if l > max {
                max = l
            }
            len += s.len(); //for fast
            v.push(s)
        }
        if len < 128 && max <= 1 { format!("[{}]", v.join(", ")) } else { format!("[\n{}]", indent(&v.join("\n"), "    ")) }
    }
}

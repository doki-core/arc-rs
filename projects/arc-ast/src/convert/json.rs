use crate::Value;
use serde_json::Number;
use std::mem::transmute;

type Json = serde_json::Value;


impl  From<Json> for Value {
    fn from(json: Json) -> Self {
       match json {
           Json::Null => Self::Null,
           Json::Bool(v) => {
               Self::Boolean(v)
           }
           Json::Number(v) => {
               v.into()
           }
           Json::String(v) => {unimplemented!()}
           Json::Array(v) => {unimplemented!()}
           Json::Object(v) => {unimplemented!()}
       }
    }
}

enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}

impl From<Number> for Value {
    fn from(n: Number) -> Self {
        let inner = unsafe {
            transmute::<Number, N>(n)
        };
        match inner {
            N::PosInt(n) => {
                n.into()
            }
            N::NegInt(n) => {
                n.into()
            }
            N::Float(n) => {
               unimplemented!()
            }
        }
    }
}


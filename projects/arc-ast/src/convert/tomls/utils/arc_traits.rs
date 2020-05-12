use arc_convert_lib::{build_dict, build_list};
use toml_edit::{Array, ArrayOfTables, Document, Item, Table, Value};

pub trait ToArc {
    fn to_arc(&self) -> String;
}

impl ToArc for Document {
    fn to_arc(&self) -> String {
        self.root.to_arc()
    }
}

impl ToArc for Item {
    fn to_arc(&self) -> String {
        match self {
            Item::None => String::from("null"),
            Item::Value(value) => value.to_arc(),
            Item::Table(table) => table.to_arc(),
            Item::ArrayOfTables(array) => array.to_arc(),
        }
    }
}

impl ToArc for Value {
    fn to_arc(&self) -> String {
        match self {
            Value::Integer(i) => format!("{}", i.value()),
            Value::String(s) => format!("{:#?}", s.value()),
            Value::Float(f) => format!("{}", f.value()),
            Value::DateTime(d) => format!("\"{}\"", d.value()),
            Value::Boolean(b) => format!("{}", b.value()),
            Value::Array(a) => format!("{}", a.to_arc()),
            Value::InlineTable(_) => {
                println!("{:#?}", self);
                unimplemented!()
            }
        }
    }
}

impl ToArc for Table {
    fn to_arc(&self) -> String {
        let mut pairs = vec![];
        for (k, v) in self.iter() {
            pairs.push(format!("{} = {}", k, v.to_arc()))
        }
        build_dict(pairs)
    }
}

impl ToArc for Array {
    fn to_arc(&self) -> String {
        let mut terms = vec![];
        for i in self.iter() {
            terms.push(i.to_arc())
        }
        build_list(terms)
    }
}

impl ToArc for ArrayOfTables {
    fn to_arc(&self) -> String {
        let mut terms = vec![];
        for i in self.iter() {
            terms.push(i.to_arc())
        }
        build_list(terms)
    }
}

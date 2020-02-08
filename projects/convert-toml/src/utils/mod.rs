use toml_edit::{Document, Item, Table, value};


trait ToArc {
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
            Item::Value(value) => {
                println!("{:#?}", value);
                unimplemented!()
            }
            Item::Table(table) => {
                table.to_arc()
            }
            Item::ArrayOfTables(array) => {
                println!("{:#?}", array);
                unimplemented!()
            }
        }
    }
}

impl ToArc for Table {
    fn to_arc(&self) -> String {
        println!("{:#?}", self);
        unimplemented!()
    }
}


#[test]
fn main() {
    let toml = r#"
"hello" = 'toml!' # comment
['a'.b]
    "#.parse::<Document>().expect("invalid doc");
    println!("{:#?}", toml.to_arc());
    unreachable!()
}



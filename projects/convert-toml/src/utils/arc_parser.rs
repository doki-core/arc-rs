use toml_edit::{Document, Item};
use crate::utils::arc_traits::ToArc;


pub fn to_arc(text: &str) -> Result<String, &'static str> {
    let toml = text.parse::<Document>().expect("invalid doc");
    let out = match &toml.root {
        Item::None => { String::from("toml = null") }
        Item::Value(v) => {
            format!("toml = {}",v)
        }
        Item::Table(table) => {
            let mut pairs = vec![];
            for (k, v) in table.iter() {
                pairs.push(format!("{} = {}", k, v.to_arc()))
            }
            pairs.join("\n")
        }
        Item::ArrayOfTables(array) => {
            array.to_arc()
        }
    };
    Ok(out)
}


#[test]
fn main() {
    let text = r#"
title = "TOML Example"

[owner]
name = "Tom Preston-Werner"
organization = "GitHub"
bio = "GitHub Cofounder & CEO\nLikes tater tots and beer."
dob = 1979-05-27T07:32:00Z # First class dates? Why not?

[database]
server = "192.168.1.1"
ports = [ 8001, 8001, 8002 ]
connection_max = 5000
enabled = true

[servers]

  # You can indent as you please. Tabs or spaces. TOML don't care.
  [servers.alpha]
  ip = "10.0.0.1"
  dc = "eqdc10"

  [servers.beta]
  ip = "10.0.0.2"
  dc = "eqdc10"
  country = "中国" # This should be parsed as UTF-8

[clients]
data = [ ["gamma", "delta"], [1, 2] ] # just an update to make sure parsers support it

# Line breaks are OK when inside arrays
hosts = [
  "alpha",
  "omega"
]

# Products

  [[products]]
  name = "Hammer"
  sku = 738594937

  [[products]]
  name = "Nail"
  sku = 284758393
  color = "gray"
    "#;
    let toml = to_arc(text);
    println!("{}", toml.unwrap_or_default());
    unreachable!()
}


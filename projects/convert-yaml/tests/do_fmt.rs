use arc_from_yaml::to_arc;

#[test]
fn main() {
    let s =
        r#"
%YAML 1.2
---
!!map {
  ? !!str "sequence"
    : !!seq [ !!str "one", !!str "two" ],
  ? !!str "mapping"
    : !!map {
    ? !!str "sky" : !!str "blue",
                      ? !!str "sea" : !!str "green",
    },
  }
"#;
    let t = to_arc(s);
    println!("{}", t.unwrap_or_default());
}

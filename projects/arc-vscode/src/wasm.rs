use arc_fmt::Settings;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rs_format(input: &str) -> String {
    let c = Settings::default();
    c.format(input)
}

#[test]
fn rs_format_test() {
    let p = rs_format(r#"{"a":1}"#);
    assert_eq!(p, "a = 1");
}

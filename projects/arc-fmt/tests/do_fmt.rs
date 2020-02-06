extern crate arc_fmt;

use arc_fmt::Settings;

#[test]
fn fmt() {
    let s = Settings::default();
    s.format_file("tests/package.json", "tests/out/package.arc");
}
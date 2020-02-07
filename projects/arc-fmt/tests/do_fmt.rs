extern crate arc_fmt;

use arc_fmt::Settings;

#[test]
fn fmt_packages() {
    let s = Settings::default();
    s.format_file("tests/package.json", "tests/out/package.arc");
}

#[test]
fn fmt_user_data() {
    let s = Settings::default();
    s.format_file("tests/user_data.json", "tests/out/user_data.arc");
}

#[test]
fn fmt_number() {
    let s = Settings::default();
    s.format_file("tests/number.arc", "tests/out/number.arc");
}

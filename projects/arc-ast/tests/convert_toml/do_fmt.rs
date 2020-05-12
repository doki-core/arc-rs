extern crate arc_from_toml;

use arc_from_toml::file_to_arc;

#[test]
fn main() -> Result<(), std::io::Error> {
    file_to_arc("tests/example.toml", "tests/out/example.arc")?;
    file_to_arc("tests/hard.toml", "tests/out/hard.arc")?;
    file_to_arc("tests/hard_unicode.toml", "tests/out/hard_unicode.arc")?;
    Ok(())
}

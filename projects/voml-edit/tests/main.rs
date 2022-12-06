// use arc_number::Number;
// use std::converter::TryInto;
use std::str::FromStr;

use diagnostic_quick::{print_errors, Failure, QResult, Success, TextStorage};

use voml_edit::VEditor;

//
#[test]
fn parse_basic() -> QResult {
    let mut store = TextStorage::default();
    let id = store.file("tests/basic.voml")?;
    let text = store.get_text(&id)?;
    let editor = VEditor::default();
    match editor.parse(text, &id) {
        Success { value: _, diagnostics } => print_errors(&store, &diagnostics)?,
        Failure { fatal, diagnostics } => {
            print_errors(&store, &diagnostics)?;
            print_errors(&store, &[fatal])?
        }
    }
    Ok(())
}

#[test]
fn te() {
    println!("{:?}", f64::from_str("1_5.0"))
}

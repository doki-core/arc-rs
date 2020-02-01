use pest_generator::derive_parser;
use std::{fs::File, io::prelude::*, path::Path};

#[test]
pub fn gen_note_down() {
    let pest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./arc.pest"));
    let rs = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./src/arc_parser.rs"));

    let derived = {
        let path = pest.to_string_lossy();
        let pest = quote! {
            #[grammar = #path]
            pub struct NoteDownParser;
        };
        derive_parser(pest, false)
    };
    let mut file = File::create(rs).unwrap();
    let out = format!("pub struct ArcParser;{}", derived);
    writeln!(file, "{}", out).unwrap();
}

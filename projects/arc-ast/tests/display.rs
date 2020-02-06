extern crate arc_ast;

use arc_ast::Arc;

#[test]
fn test() {
    println!("Null:    {}", Arc::new());
    println!("Boolean: {}", Arc::new_boolean(true));
    println!("Boolean: {}", Arc::new_boolean(false));
    println!(
        "Cite:    {}",
        Arc::new_cite(vec!["father".to_string(), "a.b".to_string()])
    );
    unreachable!()
}

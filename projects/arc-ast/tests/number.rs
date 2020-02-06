extern crate arc_ast;

use arc_ast::Arc;

#[test]
fn test() {
    let s = Arc::new_number("i32", "2147483647");
    println!("{}", s);
}

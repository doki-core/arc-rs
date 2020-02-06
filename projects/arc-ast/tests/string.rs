extern crate arc_ast;

use arc_ast::AST;

#[test]
fn test() {
    let s = AST::new_number("x0", "FFF");
    println!("{:?}", s)
}

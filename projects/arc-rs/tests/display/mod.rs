use super::*;
use arc_ast::{dict, value::Text, AST};

#[test]
fn test() {
    println!("{}", Text::from("x"));
    println!("{:?}", Value::from(Text::from("x")));
    println!("{:?}", AST::string(Text::from("x")));
    println!("{:?}", Value::from(AST::string(Text::from("x"))));
    println!("{:?}", dict! {"a": "1"});
}

//
// #[test]
// fn test() {
//     println!("Null:    {}", Arc::new());
//     println!("Boolean: {}", Arc::new_boolean(true));
//     println!("Boolean: {}", Arc::new_boolean(false));
//     println!("Cite:    {}", Arc::new_cite(vec!["father".to_string(), "a.b".to_string()]));
// }
//
// #[test]
// fn test_list() {
//     let l = list!["0", 2, 3, 1.0, 2.0, 3f64, list![false, true]];
//     assert_eq!(l[-1][-1], true);
//     assert_eq!(l.get(0).unwrap(), "0")
// }
//
// #[test]
// fn test_dict() {
//     let d = dict! {
//         "boolean": true,
//         "string": "hello world",
//         "list": list![],
//         "null": Arc::Null,
//     };
//     println!("{}", d);
// }

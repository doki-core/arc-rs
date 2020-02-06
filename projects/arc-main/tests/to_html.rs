#[macro_use]
extern crate arc_rs as arc;

use arc::Arc;

#[test]
fn test_list() {
    let l = list![1, 2, 3, 1.0, 2.0, 3f64, list![false, true]];
    println!("{}", l[0]);
}

#[test]
fn test_dict() {
    let d = dict! {
        "boolean": true,
        "string": "hello world",
        "list": list![],
        "null": Arc::Null
    };
    println!("{}", d);
}

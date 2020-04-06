#[macro_use]
extern crate arc_rs as arc;

use arc::{parse, Arc};

#[test]
fn test_list() {
    let l = list![1, 2, 3, 1.0, 2.0, 3f64, list![false, true]];
    println!("{}", l[1]);
    println!("{:?}", l.get(1));
}

#[test]
fn test_dict() {
    let d = parse(include_str!("package.arc"));
    println!("{}", d);
}

use super::*;

#[test]
fn empty() {
    let ast = parse_text(include_str!("empty.arc")).unwrap();
    assert_eq!(include_str!("empty.out.arc"), format!("{:#?}", Value::from(ast)))
}

#[test]
fn basic() {
    let ast = parse_text(include_str!("basic.arc")).unwrap();
    assert_eq!(include_str!("basic.out.arc"), format!("{:#?}", Value::from(ast)))
}

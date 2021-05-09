use serde::Serialize;
use arc_rs::ReadableConfigSerializer;
use arc_rs::Value;


#[test]
fn test_primitive() {
    let mut s = ReadableConfigSerializer::default();

    let v= Value::from(s.serialize(0usize).unwrap());
    assert_eq!(format!("{:?}", v), "0");

    let v= Value::from(s.serialize("0").unwrap());
    assert_eq!(format!("{:?}", v), "\"0\"");

    let v= Value::from(s.serialize('0').unwrap());
    assert_eq!(format!("{:?}", v), "\"0\"");

    let v= Value::from(s.serialize(true).unwrap());
    assert_eq!(format!("{:?}", v), "true");
}

//
// #[test]
// fn test_list() {
//     let mut serializer = WXFSerializer::default();
//
//     vec![0].serialize(&mut serializer).unwrap();
//     assert_eq!(serializer.to_wolfram_string(), "{0}");
//
//     vec![vec![0], vec![1]].serialize(&mut serializer).unwrap();
//     assert_eq!(serializer.to_wolfram_string(), "{{0},{1}}");
//
//     (vec![0], vec![1]).serialize(&mut serializer).unwrap();
//     assert_eq!(serializer.to_wolfram_string(), "{{0},{1}}");
// }
//
// #[derive(Serialize)]
// struct TestTuple(usize, Vec<usize>);
//
// #[derive(Serialize)]
// struct TestStruct {
//     int: usize,
//     seq: Vec<usize>,
// }
//
// #[test]
// fn test_struct() {
//     let mut serializer = WXFSerializer::default();
//     let test = TestStruct { int: 0, seq: vec![1, 2] };
//     test.serialize(&mut serializer).unwrap();
//     // let expected = r#"Test["int"->1,"seq"->{"a","b"}]"#;
//     assert_eq!(serializer.to_wolfram_string(), r#"<|"int"->0,"seq"->{1,2}|>"#);
//
//     let test = TestTuple(0, vec![1, 2]);
//     test.serialize(&mut serializer).unwrap();
//     // let expected = r#"Test["int"->1,"seq"->{"a","b"}]"#;
//     assert_eq!(serializer.to_wolfram_string(), r#"TestTuple[0,{1,2}]"#);
// }
//
// //
// // #[test]
// // fn test_enum() {
// //     #[derive(Serialize)]
// //     enum E {
// //         Unit,
// //         Newtype(u32),
// //         Tuple(u32, u32),
// //         Struct { a: u32 },
// //     }
// //
// //     let u = E::Unit;
// //     let expected = r#""Unit""#;
// //     assert_eq!(serialize(&u).unwrap(), expected);
// //
// //     let n = E::Newtype(1);
// //     let expected = r#"{"Newtype":1}"#;
// //     assert_eq!(serialize(&n).unwrap(), expected);
// //
// //     let t = E::Tuple(1, 2);
// //     let expected = r#"{"Tuple":[1,2]}"#;
// //     assert_eq!(serialize(&t).unwrap(), expected);
// //
// //     let s = E::Struct { a: 1 };
// //     let expected = r#"{"Struct":{"a":1}}"#;
// //     assert_eq!(serialize(&s).unwrap(), expected);
// // }

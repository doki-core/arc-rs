use super::*;

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            Some(s) => write!(f, "{:?}{}", self.value, s),
            None => write!(f, "{:?}", self.value),
        }
    }
}

impl Eq for NumberKind {}

//
// impl Debug for Number {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Number::Integer(n) => write!(f, "Integer({})", n),
//             Number::Integer8(n) => write!(f, "Integer8({})", n),
//             Number::Integer16(n) => write!(f, "Integer16({})", n),
//             Number::Integer32(n) => write!(f, "Integer32({})", n),
//             Number::Integer64(n) => write!(f, "Integer64({})", n),
//             Number::Integer128(n) => write!(f, "Integer128({})", n),
//             Number::Unsigned(n) => write!(f, "Unsigned({})", n),
//             Number::Unsigned8(n) => write!(f, "Unsigned8({})", n),
//             Number::Unsigned16(n) => write!(f, "Unsigned16({})", n),
//             Number::Unsigned32(n) => write!(f, "Unsigned32({})", n),
//             Number::Unsigned64(n) => write!(f, "Unsigned64({})", n),
//             Number::Unsigned128(n) => write!(f, "Unsigned128({})", n),
//             Number::Decimal(i) => write!(f, "Decimal({})", i),
//             Number::Decimal32(i) => {
//                 let mut s = unsafe {
//                     format!("{}", transmute_copy::<[u8;4], f32>(i))
//                 };
//                 if !s.ends_with('.') {
//                     s.push_str(".0")
//                 }
//                 write!(f, "Decimal32({})", s)
//             }
//             Number::Decimal64(i) => {
//                 let mut s = unsafe {
//                     format!("{}", transmute_copy::<[u8;8], f64>(i))
//                 };
//                 if !s.ends_with('.') {
//                     s.push_str(".0")
//                 }
//                 write!(f, "Decimal64({})", s)
//             }
//         }
//     }
// }
//
// impl Display for Number {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Number::Integer(i) => write!(f, "{}", i),
//             Number::Integer8(i) => write!(f, "{}i8", i),
//             Number::Integer16(i) => write!(f, "{}i16", i),
//             Number::Integer32(i) => write!(f, "{}i32", i),
//             Number::Integer64(i) => write!(f, "{}i64", i),
//             Number::Integer128(i) => write!(f, "{}i128", i),
//             Number::Unsigned(i) => write!(f, "{}", i),
//             Number::Unsigned8(i) => write!(f, "{}u8", i),
//             Number::Unsigned16(i) => write!(f, "{}u16", i),
//             Number::Unsigned32(i) => write!(f, "{}u32", i),
//             Number::Unsigned64(i) => write!(f, "{}u64", i),
//             Number::Unsigned128(i) => write!(f, "{}u128", i),
//             Number::Decimal32(i) => {
//                 let mut s = unsafe {
//                     format!("{}", transmute_copy::<[u8;4], f32>(i))
//                 };
//                 if !s.ends_with('.') {
//                     s.push_str(".0")
//                 }
//                 write!(f, "{}f32", s)
//             }
//             Number::Decimal64(i) => {
//                 let mut s = unsafe {
//                     format!("{}", transmute_copy::<[u8;8], f64>(i))
//                 };
//                 if !s.ends_with('.') {
//                     s.push_str(".0")
//                 }
//                 write!(f, "{}f64", s)
//             }
//             _ => write!(f, "{:?}", self),
//         }
//     }
// }

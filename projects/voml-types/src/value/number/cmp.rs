use num::BigInt;

use super::*;

impl PartialEq<bool> for Von {
    fn eq(&self, other: &bool) -> bool {
        match self {
            Von::Boolean(v) => v.eq(other),
            _ => false,
        }
    }
}

impl PartialEq<u8> for Von {
    fn eq(&self, other: &u8) -> bool {
        match self {
            Von::Number(v) => match v.value.to_u8() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}

impl PartialEq<u16> for Von {
    fn eq(&self, other: &u16) -> bool {
        match self {
            Von::Number(v) => match v.value.to_u16() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}
impl PartialEq<u32> for Von {
    fn eq(&self, other: &u32) -> bool {
        match self {
            Von::Number(v) => match v.value.to_u32() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}
impl PartialEq<u64> for Von {
    fn eq(&self, other: &u64) -> bool {
        match self {
            Von::Number(v) => match v.value.to_u64() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}
impl PartialEq<u128> for Von {
    fn eq(&self, other: &u128) -> bool {
        match self {
            Von::Number(v) => match v.value.to_u128() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}
impl PartialEq<usize> for Von {
    fn eq(&self, other: &usize) -> bool {
        match self {
            Von::Number(v) => match v.value.to_usize() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}
impl PartialEq<i8> for Von {
    fn eq(&self, other: &i8) -> bool {
        match self {
            Von::Number(v) => match v.value.to_i8() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}
impl PartialEq<i16> for Von {
    fn eq(&self, other: &i16) -> bool {
        match self {
            Von::Number(v) => match v.value.to_i16() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}
impl PartialEq<i32> for Von {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Von::Number(v) => match v.value.to_i32() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}
impl PartialEq<i64> for Von {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Von::Number(v) => match v.value.to_i64() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}
impl PartialEq<i128> for Von {
    fn eq(&self, other: &i128) -> bool {
        match self {
            Von::Number(v) => match v.value.to_i128() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}

impl PartialEq<isize> for Von {
    fn eq(&self, other: &isize) -> bool {
        match self {
            Von::Number(v) => match v.value.to_isize() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}

// impl PartialEq<BigInt> for Von {
//     fn eq(&self, other: &BigInt) -> bool {
//         match self {
//             Von::Number(v) => v.value.eq(other),
//             _ => false,
//         }
//     }
// }

impl PartialEq<f32> for Von {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Von::Number(v) => match v.value.to_f32() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}

impl PartialEq<f64> for Von {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Von::Number(v) => match v.value.to_f64() {
                Some(s) => s.eq(other),
                None => false,
            },
            _ => false,
        }
    }
}

use super::*;



#[derive(Clone, Eq, PartialEq)]
pub struct Text {
    handler: Option<String>,
    delimiter: TextDelimiter,
    value: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextDelimiter {
    // "
    Quotation(usize),
    // '
    Apostrophe(usize),
    /// `‹`: U+2039
    /// `›`: U+203A
    SingleAngleQuotation,
    /// `«`: U+00AB
    /// `»`: U+00BB
    DoubleAngleQuotation
}

impl Debug for Text {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.handler {
            None => (),
            Some(s) => {write!(f,"{}",s)?}
        }
        match self.delimiter {
            TextDelimiter::Quotation(n) => {
                write!(f,"{}","\"".repeat(n))?;
                write!(f,"{}",self.value)?;
                write!(f,"{}","\"".repeat(n))?;
            }
            TextDelimiter::Apostrophe(n) => {
                write!(f,"{}","\'".repeat(n))?;
                write!(f,"{}",self.value)?;
                write!(f,"{}","\'".repeat(n))?;
            }
            TextDelimiter::DoubleAngleQuotation => {
                write!(f,"«")?;
                write!(f,"{}",self.value)?;
                write!(f,"»")?;
            }
            TextDelimiter::SingleAngleQuotation => {
                write!(f,"‹")?;
                write!(f,"{}",self.value)?;
                write!(f,"»")?;
            }
        }
        Ok(())
    }
}

macro_rules! native2string {
    ($T:ty) => {
    impl From<$T> for Text {
        fn from(v: $T) -> Self {
            Self {
                handler: None,
                delimiter: TextDelimiter::Quotation(1),
                value: String::from(v)
            }
        }
    }
    };
    ($($T:ty), +) => {
        $(wrap_native!($T);)+
    };
}

macro_rules! native2value {
    ($T:ty) => {
    native2string!($T);
    impl From<$T> for Value {
        fn from(v: $T) -> Self {
            Value::String(Box::new(v.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![char, &str, String, &String];

impl From<Text> for Value {
    fn from(v: Text) -> Self {
        Value::String(Box::new(v))
    }
}

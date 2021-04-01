use super::*;

///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Integer {
    pub handler: Option<String>,
    pub value: BigInt,
}

macro_rules! native2integer {
    ($T:ty) => {
    impl From<$T> for Integer {
        fn from(v: $T) -> Self {
            Self {
                handler: None,
                value: BigInt::from(v),
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
    native2integer!($T);
    impl From<$T> for Value {
        fn from(v: $T) -> Self {
            Value::Integer(Box::new(v.into()))
        }
    }
    };
    ($($T:ty), +) => {
        $(native2value!($T);)+
    };
}

native2value![BigInt, BigUint];
native2value![u8, u16, u32, u64, u128, usize];
native2value![i8, i16, i32, i64, i128, isize];

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)?;
        match &self.handler {
            None => (),
            Some(s) => write!(f, "{}", s)?,
        }
        Ok(())
    }
}

impl Deref for Integer {
    type Target = BigInt;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<Integer> for Value {
    fn from(v: Integer) -> Self {
        Self::Integer(Box::new(v))
    }
}

impl Integer {
    ///
    pub fn set_handler(&mut self, handler: impl Into<String>) {
        self.handler = Some(handler.into())
    }
    ///
    pub fn get_handler(&self) -> Option<String> {
        self.handler.to_owned()
    }
    ///
    pub fn get_value(&self) -> BigInt {
        self.value.to_owned()
    }
    ///
    pub fn get_index(&self) -> Option<isize> {
        isize::try_from(&self.value).ok()
    }
}

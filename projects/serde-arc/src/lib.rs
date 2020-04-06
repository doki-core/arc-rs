mod ast;
mod error;
mod parse;
mod utils;

pub use ast::{
    ast_impl::{Getter, Setter},
    Arc,
};
pub use error::{Error, Result};
pub use parse::{parse, parse_file};

#[macro_export]
macro_rules! list {
    [] => ($crate::Arc::new_list());
    [$($item:expr), *] => ({
        let size = 0 $( + {let _ = $item; 1} )*;
        let mut array = std::collections::VecDeque::with_capacity(size);
        $(array.push_back($item.into());)*
        $crate::Arc::List(array)
    })
}

#[macro_export]
macro_rules! dict {
    {} => ($crate::Arc::new_dict());
    {$($key:tt: $value:expr), *} => {
        $crate::dict!($($key: $value,)*)
    };
    {$($key:tt: $value:expr,)*} => ({
        let size = 0 $( + {let _ = $key; 1} )*;
        let mut map = linked_hash_map::LinkedHashMap::with_capacity(size);
        $(map.insert($key.into(), $value.into());)*
        $crate::Arc::Dict(map)
    })
}

extern crate arc_ast;

pub use arc_ast::Arc;

#[macro_export]
macro_rules! list {
    [] => ($crate::Arc::new_list());
    [$($item:expr), *] => ({
        let size = 0 $( + {let _ = $item; 1} )*;
        let mut array = Vec::with_capacity(size);
        $(array.push($item.into());)*
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
        use std::collections::HashMap;
        let size = 0 $( + {let _ = $key; 1} )*;
        let mut map = HashMap::with_capacity(size);
        $(map.insert($key.into(), $value.into());)*
        $crate::Arc::Dict(map)
    })
}

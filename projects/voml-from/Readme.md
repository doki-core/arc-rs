无后效宏 @
后效宏 #


```xo
class String {

}

type RefinedInt: Int {
    self > 0
}

type LowerCase: String {
    T <: Clone + Debug
    self == self.to_lowercase()
}

type StringUTF8: String {
    self == self.to_utf8()
}

type StringUTF16: String {
    self == self.to_utf16()
}

type StringASCII: String {
    self == self.to_ascii()
}

type String: StringASCII {
    self.to_ascii()
    @cast;
    @cast_unsafe;
    from(value) {
        value.to_ascii()
    }
}

type Record : [
    a: b
] {


}


type Odd: usize {
    self % 2 == 1
    cast { value }
}

type Even: usize {
    self % 2 == 0
    cast { value }
}

def odd_inc(mut x: int) {
    ensure MyError("not odd", @caller) {
        @input(x)
    }
    ensure "" {
        x == @input(x) + 2
    }
}
type CatPath: Path {
    
    cast {
        value.pas
    }
}

"a" as!! UTF8Unsafe
```

u8 =!> u16 =?> u32 =!!>




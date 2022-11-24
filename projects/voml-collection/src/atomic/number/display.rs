use super::*;

struct WrapDisplay<'i> {
    inner: &'i BigFloatNumber,
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Number").field("kind", &self.hint).field("value", &WrapDisplay { inner: &self.value }).finish()
    }
}

impl Debug for WrapDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner.to_string())
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.hint)
    }
}

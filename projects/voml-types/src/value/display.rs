use super::*;

impl Debug for Von {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Von::Integer(v) => Debug::fmt(v, f),
            Von::Decimal(v) => Debug::fmt(v, f),
            Von::List(v) => Debug::fmt(v, f),
            Von::Dict(v) => Debug::fmt(v, f),
        }
    }
}

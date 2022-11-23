use super::*;
impl Error for VError {}

impl Debug for VErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VErrorKind::IOError(v) => Debug::fmt(v, f),
            VErrorKind::ParseError(v) => Debug::fmt(v, f),
            VErrorKind::Duplicate(v) => Debug::fmt(v, f),
            VErrorKind::Custom(v) => f.write_str(&v),
        }
    }
}
impl Display for VError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Display for VErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<std::io::Error> for VError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: Box::new(VErrorKind::IOError(e)), level: DiagnosticLevel::Error, file: Default::default() }
    }
}
// impl From<std::fmt::Error> for VomlError {
//     fn from(e: std::fmt::Error) -> Self {
//         Self { kind: Box::new(VomlErrorKind::FormatError(e)), level: DiagnosticLevel::None, file: None, range: None }
//     }
// }

impl From<Infallible> for VError {
    fn from(_: Infallible) -> Self {
        Self { kind: Box::new(VErrorKind::Custom("".to_string())), level: DiagnosticLevel::Error, file: Default::default() }
    }
}

impl From<()> for VError {
    fn from(_: ()) -> Self {
        Self { kind: Box::new(VErrorKind::Custom("".to_string())), level: DiagnosticLevel::Error, file: Default::default() }
    }
}

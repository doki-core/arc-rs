use super::*;

impl Debug for VomlErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VomlErrorKind::IOError(v) => Debug::fmt(v, f),
            VomlErrorKind::ParseError(v) => Debug::fmt(v, f),
            VomlErrorKind::Duplicate(v) => Debug::fmt(v, f),
            VomlErrorKind::UnknownError => f.write_str("UnknownError"),
        }
    }
}

impl From<std::io::Error> for VomlError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: Box::new(VomlErrorKind::IOError(e)), level: DiagnosticLevel::Error, file: Default::default() }
    }
}
// impl From<std::fmt::Error> for VomlError {
//     fn from(e: std::fmt::Error) -> Self {
//         Self { kind: Box::new(VomlErrorKind::FormatError(e)), level: DiagnosticLevel::None, file: None, range: None }
//     }
// }

impl From<Infallible> for VomlError {
    fn from(_: Infallible) -> Self {
        Self { kind: Box::new(VomlErrorKind::UnknownError), level: DiagnosticLevel::Error, file: Default::default() }
    }
}

impl From<()> for VomlError {
    fn from(_: ()) -> Self {
        Self { kind: Box::new(VomlErrorKind::UnknownError), level: DiagnosticLevel::Error, file: Default::default() }
    }
}

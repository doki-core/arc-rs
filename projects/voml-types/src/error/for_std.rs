use super::*;

impl Error for VError {}

impl Debug for VError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for VError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<std::io::Error> for VError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: Box::new(VErrorKind::IOError { error: e, file: "".to_string() }), level: DiagnosticLevel::Error }
    }
}

impl From<Infallible> for VError {
    fn from(_: Infallible) -> Self {
        Self { kind: Box::new(VErrorKind::Custom("".to_string())), level: DiagnosticLevel::Error }
    }
}

impl From<()> for VError {
    fn from(_: ()) -> Self {
        Self { kind: Box::new(VErrorKind::Custom("".to_string())), level: DiagnosticLevel::Error }
    }
}

#[allow(dead_code)]
pub struct Serializer<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
}

#[derive(Clone, Debug)]
pub struct CompactFormatter;

#[derive(Clone, Debug)]
pub struct PrettyFormatter;

use super::Sting;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Datum {
    I8(i8),
    String(Sting),
}

use super::Sting;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Datum {
    I8(i8),
    String(Sting),
}

impl From<i8> for Datum{
    fn from(value: i8) -> Self {
        Datum::I8(value)
    }
}
impl From<Sting> for Datum{
    fn from(value: Sting) -> Self {
        Datum::String(value)
    }
}
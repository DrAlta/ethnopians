use super::super::super::Sting;
use super::DatumTypeInfo;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DatumType {
    I8,
    String,
}

impl DatumType {
    #[allow(private_bounds)]
    pub fn get<T>() -> DatumType
    where
        DatumType: DatumTypeInfo<T>,
    {
        <DatumType as DatumTypeInfo<T>>::datum_type_info()
    }
}

impl DatumTypeInfo<i8> for DatumType {
    fn datum_type_info() -> DatumType {
        DatumType::I8
    }
}
impl DatumTypeInfo<Sting> for DatumType {
    fn datum_type_info() -> DatumType {
        DatumType::String
    }
}

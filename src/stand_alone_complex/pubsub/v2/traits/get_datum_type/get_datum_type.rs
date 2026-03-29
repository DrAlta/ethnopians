use super::{super::super::Sting, DatumType};

pub trait GetDatumType<const INDEX: usize> {
    fn get_data_type(&self) -> Option<DatumType>;
}

impl GetDatumType<0> for Vec<(i8, i8)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::I8)
    }
}
impl GetDatumType<1> for Vec<(i8, i8)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::I8)
    }
}

impl GetDatumType<0> for Vec<(i8, Sting)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::I8)
    }
}
impl GetDatumType<1> for Vec<(i8, Sting)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::String)
    }
}

impl GetDatumType<0> for Vec<(Sting, Sting)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::String)
    }
}
impl GetDatumType<1> for Vec<(Sting, Sting)> {
    fn get_data_type(&self) -> Option<DatumType> {
        Some(DatumType::String)
    }
}

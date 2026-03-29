use crate::stand_alone_complex::pubsub::v2::DatumType;

use super::super::Datum;
pub trait GetColumnsDatums {
    fn get_column<'a>(&'a self, index: usize) -> Option<Vec<Datum>>;
    fn get<'a>(&'a self, field: &str) -> Option<Vec<Datum>>;

    fn datum_type_info(&self, index: usize) -> Option<DatumType>;
    fn lookup_feilds_type(&self, k: &str) -> Option<DatumType>;
    //fn get_i8_iter<'a>(&'a self, index: usize) -> Option<impl Iterator<Item = &'a i8>>;
    //fn get_string_iter<'a>(&'a self, index: usize) -> Option<impl Iterator<Item = &'a Sting>>;
}

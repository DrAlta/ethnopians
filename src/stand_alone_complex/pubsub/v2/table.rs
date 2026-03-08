use std::sync::Arc;

use super::{empty_i8, empty_string, Column, Datum, DatumType, GetColumnsValues, Sting};

#[derive(Debug)]
pub struct Table<const SIZE: usize> {
    field_names: [Sting; SIZE],
    fields: [Column; SIZE],
}
impl GetColumnsDatums for Vec<(i8, i8)> {
    fn get_column<'a>(&'a self, index: usize) -> Option<Vec<Datum>> {
        match index {
            0 => Some(self.iter().map(|(a, _b)| Datum::I8(*a)).collect()),
            1 => Some(self.iter().map(|(_a, b)| Datum::I8(*b)).collect()),
            _ => None,
        }
    }
    
    fn datum_type_info(&self, index: usize) -> Option<DatumType> {
        match index {
            0 => Some(DatumType::I8),
            1 => Some(DatumType::I8),
            _ => None,
        }
    }
    
    fn get<'a>(&'a self, field: &str) -> Option<Vec<Datum>> {
        todo!()
    }
}
use super::super::{Datum, Sting};
pub trait GetColumnsValues<const INDEX: usize> {
    fn get_column<'a>(&'a self) -> Vec<Datum>;
    fn get_i8_iter<'a>(&'a self) -> impl Iterator<Item = &'a i8>;
    fn get_string_iter<'a>(&'a self) -> impl Iterator<Item = &'a Sting>;
}
impl GetColumnsValues<0> for Vec<(i8, i8)> {
    fn get_column<'a>(&'a self) -> Vec<Datum> {
        self.iter().map(|(a, _b)| Datum::I8(*a)).collect()
    }

    fn get_i8_iter<'a>(&'a self) -> impl Iterator<Item = &'a i8> {
        self.iter().map(|(a, _b)| a)
    }

    fn get_string_iter<'a>(&'a self) -> impl Iterator<Item = &'a Sting> {
        [].iter()
    }
}
impl GetColumnsValues<1> for Vec<(i8, i8)> {
    fn get_column<'a>(&'a self) -> Vec<Datum> {
        self.iter().map(|(_a, b)| Datum::I8(*b)).collect()
    }

    fn get_i8_iter<'a>(&'a self) -> impl Iterator<Item = &'a i8> {
        self.iter().map(|(_a, b)| b)
    }

    fn get_string_iter<'a>(&'a self) -> impl Iterator<Item = &'a Sting> {
        [].iter()
    }
}

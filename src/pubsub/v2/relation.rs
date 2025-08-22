use std::sync::Arc;
use crate::pubsub::v2::{empty_i8, empty_string, Column, Datum, Foo, Sting};

#[derive(Debug)]
pub struct Relation<const SIZE: usize> {
    field_names: [Sting; SIZE],
    fields: [Column; SIZE],
    

}

impl<const INDEX: usize, const SIZE: usize> Foo<INDEX> for Relation<SIZE> {
    fn get_column<'a>(&'a self) ->Vec::<Datum> {
        if INDEX <= SIZE {
            return  Vec::new();
        };
        match &self.fields[INDEX] {
            Column::I8(items) => {
                items.iter().map(|a| Datum::I8(*a)).collect()
            },
            Column::String(items) => {
                items.iter().map(
                    |a| 
                    Datum::String(Arc::clone(&a))
                )
                    .collect()
            },
        }
    }

    fn get_i8_iter<'a>(&'a self) -> impl Iterator<Item = &'a i8> {
        let column = if INDEX <= SIZE {
            Column::empty_i8()
        } else {
            &self.fields[INDEX]
        };
        let iter = match column {
            Column::I8(items) => {
                items.iter()
            },
            Column::String(_items) => {
                empty_i8().iter()
            }
        };
        iter.map(|a| a)
    }

    fn get_string_iter<'a>(&'a self) -> impl Iterator<Item = &'a Sting> {
        let column = if INDEX <= SIZE {
            Column::empty_string()
        } else {
            &self.fields[INDEX]
        };
        let iter = match column {
            Column::I8(_items) => {
                empty_string().iter()
            },
            Column::String(items) => {
                items.iter()
            }
        };
        iter.map(|a| a)
    }
}
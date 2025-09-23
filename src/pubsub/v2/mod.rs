use std::{sync::Arc};
use std::collections::HashMap;

mod column;
pub use column::{Column, empty_i8, empty_string}; 
mod datum;
pub use datum::Datum;
mod get_datum_type;
pub use get_datum_type::{DatumType, GetDatumType};
mod relation;
pub use relation::Relation;

pub mod foo;
pub use foo::Foo;

type Sting = Arc<str>;

pub fn join_i8<const AI: usize, Id: Eq + std::hash::Hash, A: Foo<AI>>(
    working: &mut HashMap<Id, i8>,
    a: &A,
) {
    let temp: Vec<i8> = a.get_i8_iter().map(|x| *x).collect();
    working.retain(|_, v| temp.contains(&*v));
}

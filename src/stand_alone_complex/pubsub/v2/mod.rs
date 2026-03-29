use std::collections::HashMap;
use std::sync::Arc;

mod traits;
pub use traits::{DatumType, GetColumnsDatums, GetColumnsValues, GetDatumType};
mod column;
pub use column::{empty_i8, empty_string, Column};
mod datum;
pub use datum::Datum;
mod join;
pub use join::{join, new_join};
mod relation;
pub use relation::Relation;

type Sting = Arc<str>;

pub fn join_i8<const AI: usize, Id: Eq + std::hash::Hash, A: GetColumnsValues<AI>>(
    working: &mut HashMap<Id, i8>,
    a: &A,
) {
    let temp: Vec<i8> = a.get_i8_iter().map(|x| *x).collect();
    working.retain(|_, v| temp.contains(&*v));
}

//mod table;
//pub use table::Table;

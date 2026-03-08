use std::collections::HashMap;

use qol::logy;

use crate::stand_alone_complex::pubsub::v2::{join::package_fields, traits::GetColumnsDatums};

use super::{intersect_values, Lookup, super::Datum};

pub fn new_join<const N: usize, M: Lookup + DB>(
    variables: [Vec<((&str, &str), (&str, &str))>; N],
    db: &M,
) -> Result<Vec<[Datum; N]>, String> 

{
    let matched_types: [_; N] = std::array::from_fn(|i| {
        let variable = &variables[i];
        let mut x = variable.iter();
        let Some(((table_id_a, field_id_a), _)) = x.next() else {
            logy!("error", "failed to get first of variables");
            return false;
        };
        let Some(datum_type) = db.lookup_relations_fields_type(table_id_a, field_id_a) else {
            logy!(
                "error",
                "failed to look uptables typetable_id:{table_id_a:?}, field_id:{field_id_a:?}"
            );
            return false;
        };
        x.all(|((table_id_a, field_id_a), (table_id_b, field_id_b))| {
            let Some(this_datum_type_a) = db.lookup_relations_fields_type(table_id_a, field_id_a)
            else {
                return false;
            };
            if this_datum_type_a != datum_type {
                return false;
            };

            let Some(this_datum_type_b) = db.lookup_relations_fields_type(table_id_b, field_id_b)
            else {
                return false;
            };
            this_datum_type_b == datum_type
        })
    });
    if !matched_types.into_iter().all(|t| t) {
        logy!("error", "(matched_types{matched_types:?}");
        return Err(format!("{}:{}:relations types mismatched", file!(), line!()));
    }

    let final_fields: [_; N] = std::array::from_fn(|i| {
        let mut working;
        let variable = &variables[i];
        let mut iter = variable.into_iter();
        let Some((first, _)) = iter.next() else {
            logy!("error", "failed to get first");
            return Vec::new();
        };
        /*
        let Some(datum_type) = db.lookup_relations_fields_type(first.0, first.1) else {
            logy!(
                "error",
                "failed to lookup table type of table_id:{:?} field_id:{}",
                first.0,
                first.1
            );
            return Vec::new();
        };
        */
        

        let a1 = db.get(first.0).unwrap();
        let Some(x) = a1.get(first.1) else {
            return Vec::new();
        };
        working = x.into_iter().enumerate().map(|(k, v)| (k, v)).collect();
        
        

        for term in iter {
            let a1 = db.get(term.0 .0).unwrap();
            let Some(x) = a1.get(first.1) else {
                return Vec::new();
            };
            intersect_values(&mut working, &x)
        }
            
        working.into_iter().map(|(_, v)| v).collect()
    });


    Ok(package_fields(final_fields))
}


pub trait DB {
    fn get (&self, k: &str) -> Option<&Box<dyn GetColumnsDatums>>;
}

impl DB for HashMap<String, Box<dyn GetColumnsDatums>>{
    fn get (&self, k: &str) -> Option<&Box<dyn GetColumnsDatums>> {
        self.get(k)
    }
}
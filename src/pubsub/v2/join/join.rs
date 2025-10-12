use std::collections::HashMap;

use qol::logy;

use crate::pubsub::v2::{join::foo2, Datum, DatumType, Relation};

use super::Lookup;

pub fn join<const SIZE: usize, const N: usize>(
    variables: [Vec<((&str, &str), (&str, &str))>; N],
    db: &HashMap<String, Relation<SIZE>>,
) -> Result<Vec<[Datum; N]>, &'static str> {
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
        return Err("relations types mismatched");
    }

    let mut final_fields: [_; N] = std::array::from_fn(|i| {
        let mut working;
        let variable = &variables[i];
        let mut iter = variable.into_iter();
        let Some((first, _)) = iter.next() else {
            logy!("error", "failed to get first");
            return Vec::new();
        };
        let Some(datum_type) = db.lookup_relations_fields_type(first.0, first.1) else {
            logy!(
                "error",
                "failed to lookup table type of table_id:{:?} field_id:{}",
                first.0,
                first.1
            );
            return Vec::new();
        };

        match datum_type {
            DatumType::I8 => {
                let a1 = db.get(first.0).unwrap();
                let Some(x) = a1.get_i8_iter(first.1) else {
                    return Vec::new();
                };
                working = x.enumerate().map(|(k, v)| (k, v.clone())).collect();
            }
            DatumType::String => return Vec::new(), /*{
                                                        let a1 = db.get(first.0).unwrap();
                                                        let Some(x) = a1.get_string_iter(first.1) else {
                                                            return Vec::new()
                                                        };
                                                        working = x
                                                            .enumerate()
                                                            .map(|(k, v)| (k, v.clone()))
                                                            .collect();
                                                    },*/
        };
        for term in iter {
            match datum_type {
                DatumType::I8 => {
                    let a1 = db.get(term.0 .0).unwrap();
                    let Some(x) = a1.get_i8_iter(first.1) else {
                        return Vec::new();
                    };
                    foo2(&mut working, &x.map(|x| *x).collect())
                }
                DatumType::String => (), /* {
                                             let a1 = db.get(first.0).unwrap();
                                             let Some(x) = a1.get_string_iter(first.1) else {
                                                 return Vec::new()
                                             };
                                             working = x
                                                 .enumerate()
                                                 .map(|(k, v)| (k, v.clone()))
                                                 .collect();
                                         },*/
            };
        }
        working.into_iter().map(|(_, v)| Datum::I8(v)).collect()
    });

    let mut ret = Vec::new();
    for _ in 0..final_fields[0].len() {
        let x: [Datum; N] = std::array::from_fn(|i| final_fields[i].pop().unwrap());
        ret.push(x)
    }
    Ok(ret)
}

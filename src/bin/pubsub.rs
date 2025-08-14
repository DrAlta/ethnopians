use std::collections::HashMap;

use ethnolib::pubsub::v2::{Datum, DatumType, Foo, GetDatumType};

fn main() {
    let a = HashMap::from([
        (
            "Table1". to_owned(), 
            Relation{
                a_title: "A".to_owned(),
                b_title: "B".to_owned(),
                tuples: vec![(1, 2), (2, 3), (4, 5), (4, 6), (2, 42)]
            }
        )
    ]);
    println!(
        "{:?}",
        join(
            [vec![
                (
                    "A".to_owned(),
                    "B".to_owned(),
                )
            ]],
            &a
        )
    );
}

fn join<const N: usize, T: Foo<0> + Foo<1> + GetDatumType<0> + GetDatumType<1>>(
    variables: [Vec<(String, String)>; N], 
    db: &HashMap<String, Relation<T>>
) -> Result<Vec<[Datum;N]>, &'static str>{

    let matched_types: [_; N] = std::array::from_fn(
        |i|
        {
            let variable = &variables[i];
            let mut x = variable.iter();
            let Some((table_id, field_id)) = x.next() else {
                return false
            };
            let Some(datum_type) = look_tables_type(table_id,field_id,db) else {
                return false
            };
            x.all(|(table_id, field_id)|{
            let Some(this_datum_type) = look_tables_type(table_id,field_id,db) else {
                return false
            };
            this_datum_type == datum_type
            })
        }
    ); 
    if !matched_types.into_iter().all(|t| t) {
        return Err("relations types mismatched")
    }
    
    let mut final_fields: [_; N] = std::array::from_fn(
        |i|
        {
            let mut working;
            let variable = &variables[i];
            let mut iter = variable.into_iter();
            let Some((first_relation, first_field)) = iter.next() else {
                return Vec::new();
            };
            let Some(datum_type) = look_tables_type(&first_relation, &first_field, db) else {
                return Vec::new()
            };

            match datum_type {
                DatumType::I8 => {
                    let a1 = db.get(first_relation).unwrap();
                    if &a1.a_title == first_field {
                        working = Foo::<0>::get_i8_iter(&a1.tuples).enumerate().map(|(k,v)| (k, v.clone())).collect();
                    } else {
                        working = Foo::<1>::get_i8_iter(&a1.tuples).enumerate().map(|(k,v)| (k, v.clone())).collect();
                    }
                },
                DatumType::String => todo!(),
            };
            for term in iter {
                match datum_type {
                    DatumType::I8 => {
                        let rel = db.get(&term.0).unwrap();
                        if rel.a_title == term.1 {
                            let a = Foo::<0>::get_i8_iter(&rel.tuples).map(|x|*x).collect();
                            foo2(&mut working, &a)
                        }else {
                            assert_eq!(rel.b_title, term.1);
                            let a = Foo::<1>::get_i8_iter(&rel.tuples).map(|x|*x).collect();
                            foo2(&mut working, &a)
                        }
                    },
                    DatumType::String => todo!(),
                }
            }
            working
            .into_iter()
            .map(|(_,v)|Datum::I8(v))
            .collect()
        }
    );

    let mut ret = Vec::new();
    for _ in 0 .. final_fields[0].len(){
        let x: [Datum; N] = std::array::from_fn(
            |i| 
            final_fields[i].pop().unwrap()
        );
        ret.push(x)
    }
    Ok(ret)

}

fn look_tables_type<T: Foo<0> + Foo<1> + GetDatumType<0> + GetDatumType<1>>(table_id: &String, field_id: &String, db: &HashMap<String, Relation<T>>) -> Option<DatumType> {
    let table = db.get(table_id)?;
    let datum_type = if &table.a_title == field_id {
        GetDatumType::<0>::get_data_type(&table.tuples)?

    } else if &table.a_title == field_id {
        GetDatumType::<1>::get_data_type(&table.tuples)?
    }else {
        return None
    };
    Some(datum_type)
}



#[derive(Debug)]
pub struct Relation<T: Foo<0> + Foo<1>> {
    pub a_title: String,
    pub b_title: String,
    pub tuples: T,
}

fn foo2(working: &mut HashMap<usize, i8>, a: &Vec<i8>){
    working.retain(
        |_,v|
        a.contains(v)
    );
}

/*
use ethnolib::pubsub::event::{join, Relation};

fn main() {
    let a = HashMap::from([(
        "Table1".to_owned(),
        Relation {
            titles: ["A".to_owned(), "B".to_owned()],
            tuples: vec![[1, 2], [2, 3], [4, 5], [4, 6]],
        },
    )]);
    println!(
        "{:?}",
        join(
            &a,
            vec![
                ("Table1".to_owned(), "A".to_owned()),
                ("Table1".to_owned(), "B".to_owned())
            ]
        )
    );
}

*/
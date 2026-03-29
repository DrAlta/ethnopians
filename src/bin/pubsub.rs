use std::collections::{BTreeSet, HashMap};

use ethnolib::stand_alone_complex::pubsub::v2::{
    join, new_join, Column, GetColumnsDatums, GetColumnsValues, Relation,
};
use qol::assert_specimen;

fn main() {
    let a = HashMap::from([(
        "Table1".to_owned(),
        Relation::new(
            ["A".into(), "B".into()],
            [
                Column::I8(vec![1, 2, 4, 4, 2]),
                Column::I8(vec![2, 3, 5, 6, 42]),
            ],
        ),
    )]);

    let x: BTreeSet<_> = join([vec![(("Table1", "A"), ("Table1", "B"))]], &a)
        .unwrap()
        .into_iter()
        .collect();
    println!("{x:?}",);

    let db: HashMap<String, Box<dyn GetColumnsDatums>> = HashMap::from([(
        "Table1".to_owned(),
        Box::new(Relation::new(
            ["A".into(), "B".into()],
            [
                Column::I8(vec![1, 2, 4, 4, 2]),
                Column::I8(vec![2, 3, 5, 6, 42]),
            ],
        )) as Box<dyn GetColumnsDatums>,
    )]);

    let y: BTreeSet<_> = new_join([vec![(("Table1", "A"), ("Table1", "B"))]], &db)
        .unwrap()
        .into_iter()
        .collect();

    assert_specimen!(&x, &y);
    use ethnolib::stand_alone_complex::pubsub::v2::Datum::I8;

    assert_specimen!(
        x,
        BTreeSet::from([[I8(4)], [I8(1)], [I8(2)], [I8(4)], [I8(2)]])
    )
}

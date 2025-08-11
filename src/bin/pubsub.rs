use std::collections::HashMap;

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

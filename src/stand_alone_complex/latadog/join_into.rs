use std::rc::Rc;

use crate::stand_alone_complex::latadog::{join_helper, GetKey, Relation, Variable};

pub fn join_into<'a, 'b, Key: Ord, const INDEX1: usize, Tuple1, const INDEX2: usize, Tuple2, V>(
    input1: &'a Variable<INDEX1, Key, Tuple1>,
    input2: &'b Variable<INDEX2, Key, Tuple2>,
    output: &mut Variable<0, Key, (Key, V)>,
    mut logic: impl FnMut(&Key, &Tuple1, &Tuple2) -> (Key, V),
) where
    Relation<INDEX1, Key, Tuple1>: GetKey<INDEX1, Key>,
    Relation<INDEX2, Key, Tuple2>: GetKey<INDEX2, Key>,
{
    let mut results = Vec::new();

    println!("input1.recent and input2.stable.");
    for batch2 in input2.stable.iter() {
        println!("join_into{}", line!());
        join_helper(&input1.recent, &batch2, |k, v1, v2| {
            println!("join_into{}", line!());
            results.push(logic(k, v1, v2))
        });
    }

    println!("input1.stable and input2.recent.");
    for batch1 in input1.stable.iter() {
        join_helper(&batch1, &input2.recent, |k, v1, v2| {
            results.push(logic(k, v1, v2))
        });
    }

    println!("input1.recent and input2.recent.");
    join_helper(&input1.recent, &input2.recent, |k, v1, v2| {
        results.push(logic(k, v1, v2))
    });

    let x = Rc::new(results).into();
    output.insert(x);
}

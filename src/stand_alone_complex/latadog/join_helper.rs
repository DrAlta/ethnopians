use crate::stand_alone_complex::latadog::{gallop, GetKey, Relation};

pub fn join_helper<'a, 'b, const INDEX1: usize, const INDEX2: usize, Key: Ord, Tuple1, Tuple2>(
    input1: &'a Relation<INDEX1, Key, Tuple1>,
    input2: &'b Relation<INDEX2, Key, Tuple2>,
    mut result: impl FnMut(&Key, &Tuple1, &Tuple2),
) where
    Relation<INDEX1, Key, Tuple1>: GetKey<INDEX1, Key>,
    Relation<INDEX2, Key, Tuple2>: GetKey<INDEX2, Key>,
{
    // represent the relations as slices.
    let mut slice1 = &input1.index[..];
    let mut slice2 = &input2.index[..];
    println!("join_helper{}\ns1:{slice1:?}\ns2:{slice2:?}", line!());

    while !slice1.is_empty() && !slice2.is_empty() {
        println!("join_helper{}", line!());

        use std::cmp::Ordering;

        // If the keys match call `result`, else advance the smaller key until they might.
        match input1.get(0).cmp(input2.get(0)) {
            Ordering::Less => {
                println!("Ordering::Less{}", line!());

                slice1 = gallop(slice1, |x| input1.get(*x) < input2.get(0));
            }
            Ordering::Equal => {
                println!("Ordering::Equal{}", line!());

                // Determine the number of matching keys in each slice.
                let count1 = slice1
                    .iter()
                    .take_while(|&&x| input1.get(x) == input1.get(0))
                    .count();
                let count2 = slice2
                    .iter()
                    .take_while(|&&x| input2.get(x) == input2.get(0))
                    .count();

                println!("c1:{count1}\nc2:{count2}");
                // Produce results from the cross-product of matches.
                for index1 in 0..count1 {
                    for index2 in 0..count2 {
                        result(
                            input1.get(slice1[index1]),
                            &input1.elements[slice1[index1]],
                            &input2.elements[slice2[index2]],
                        );
                    }
                }

                // Advance slices past this key.
                slice1 = &slice1[count1 + 1..];
                slice2 = &slice2[count2 + 1..];
            }
            Ordering::Greater => {
                println!("Ordering::Greater{}", line!());
                slice2 = gallop(slice2, |x| input2.get(*x) < input1.get(0));
            }
        }
    }
}

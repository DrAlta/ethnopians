use crate::stand_alone_complex::pubsub::v2::Datum;

pub fn package_fields<const N: usize>(mut final_fields: [Vec<Datum>; N]) -> Vec<[Datum; N]> {
    let mut ret = Vec::new();
    for _ in 0..final_fields[0].len() {
        let x: [Datum; N] = std::array::from_fn(|i| final_fields[i].pop().unwrap());
        ret.push(x)
    }
    ret
}

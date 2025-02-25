use std::{collections::HashMap, num::NonZero};

use thats_so_random::RandomNumberGenerator;

use super::Trait;

pub fn sample<RNG: RandomNumberGenerator>(
    a: &HashMap<Trait, NonZero<u8>>,
    exclude: &Vec<Trait>,
    rng: &mut RNG,
) -> Option<Trait> {
    let mut x = rng.random_range(0, a.iter().map(|(_, x)| x.get() as i32).sum());
    for (r#trait, i) in a {
        let i = i.get() as i32;
        if x < i && !exclude.contains(r#trait) {
            return Some(r#trait.clone());
        } else {
            x = x - i;
        }
    }
    Some(a.iter().next()?.0.clone())
}
#[cfg(test)]
mod tests {

    use qol::pout;

    use super::*;
    #[test]
    fn sample_test() {
        let mut rng = thats_so_random::DummyRNG::new(
            [0_u32, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        let a = HashMap::from([
            ("a".to_owned(), NonZero::new(1_u8).unwrap()),
            ("b".to_owned(), NonZero::new(2).unwrap()),
            ("c".to_owned(), NonZero::new(3).unwrap()),
            ("d".to_owned(), NonZero::new(4).unwrap()),
            ("e".to_owned(), NonZero::new(5).unwrap()),
        ]);

        pout!("{:?}", sample(&a, &Vec::new(), &mut rng))
    }
}

use std::{collections::HashMap, num::NonZero};

use qol::pout;
use thats_so_random::RandomNumberGenerator;

use super::{sample, Trait};

pub struct Client {
    desired_traits: [Trait; 3],
    enjoyment_of_trait: [f32; 3],
    importance_of_trait: [f32; 3],
    enjoyment_of_fullfillment: f32,
}

impl Client {
    /// when a client rates a cuddlebuddy. multiply the girls traits that the clients desire by the clients weight for that trait then sum the,
    /// their is a virtual traits that each trait that the girls has is 1.0 and any thay she doesn'  in 0.0 then you weight that by how important that trait is to the pclients
    pub fn rate_server(&self, server: &HashMap<Trait, f32>) -> f32 {
        let (enjoyment, fulfillment) =
            (0..3_usize).fold((0.0, 0.0), |(mut enjoyment, mut fulfillment), idx| {
                let Some(trait_value) = server.get(&self.desired_traits[idx]) else {
                    return (enjoyment, fulfillment);
                };
                enjoyment += trait_value * self.enjoyment_of_trait[idx];
                fulfillment += self.importance_of_trait[idx];
                (enjoyment, fulfillment)
            });
        enjoyment + (fulfillment * self.enjoyment_of_fullfillment)
    }
    pub fn gen<RNG: RandomNumberGenerator>(traits: &HashMap<Trait, NonZero<u8>>, rng: &mut RNG) -> Client {
        let mut exclude = Vec::new();
        exclude.push(sample(traits, &exclude, rng).unwrap());
        exclude.push(sample(traits, &exclude, rng).unwrap());
        exclude.push(sample(traits, &exclude, rng).unwrap());
        Self {
            desired_traits: [
                exclude.pop().unwrap(),
                exclude.pop().unwrap(),
                exclude.pop().unwrap(),
            ],
            enjoyment_of_trait: [rng.random(), rng.random(), rng.random()],
            importance_of_trait: [rng.random(), rng.random(), rng.random()],
            enjoyment_of_fullfillment: rng.random(),
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    let mut rng = thats_so_random::DummyRNG::new([0_u32, 1, 2,3,4]);

    let traits = HashMap::from([
        ("a".to_owned(), NonZero::new(1).unwrap()),
        ("b".to_owned(), NonZero::new(1).unwrap()),
        ("c".to_owned(), NonZero::new(1).unwrap()),
    ]);

    let client = Client::gen(&traits, &mut rng);
    let server = HashMap::from([
        ("a".to_owned(), 1.0),
        ("b".to_owned(), 1.0),
        ("c".to_owned(), 1.0),
    ]);

    pout!("{}", client.rate_server(&server));
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test() {
        main();
    }
}

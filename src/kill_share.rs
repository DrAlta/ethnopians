use std::collections::HashMap;

type UnitId = u16;

fn mult_part(hash: &mut HashMap<UnitId, f32>, heal: f32) {
    let total: f32 = hash.values().sum();
    // is heal is greater that the total then there will be no damage left so we just empty the hashmap.
    if heal >= total {
        *hash = HashMap::new();
        return;
    };
    let target = total - heal;
    let mult = target / total;
    for (_, x) in hash {
        *x *= mult;
    }
}

pub fn heal(hash: &mut HashMap<UnitId, f32>, heal: f32) {
    mult_part(hash, heal);
    hash.retain(|_k, v| v >= &mut 1.0);
}

pub fn calc_share(hash: &HashMap<UnitId, f32>) -> HashMap<UnitId, f32> {
    let total: f32 = hash.values().sum();
    hash.iter().map(|(k, v)| (k.clone(), v / total)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mult_part_test() {
        let mut hash = HashMap::from([(1, 2.0), (2, 2.0), (3, 2.0), (4, 2.0)]);
        mult_part(&mut hash, 5.0);
        assert_eq!(
            hash,
            HashMap::from([(1, 0.75), (2, 0.75), (3, 0.75), (4, 0.75),])
        )
    }
    #[test]
    fn heal_test() {
        let mut hash = HashMap::from([(1, 2.0), (2, 2.0), (3, 8.0), (4, 2.0), (5, 2.0)]);
        heal(&mut hash, 10.0);
        assert_eq!(hash, HashMap::from([(3, 3.0),]))
    }
    #[test]
    fn kill_share_devalues_damage_done_before_healing_test() {
        // the hypothical situat is a unit take has 9 HP and in broth down to 1 hp
        let mut hash = HashMap::from([(1, 2.0), (2, 2.0), (3, 2.0), (4, 2.0)]);
        //then heals 4 to being her back to 5
        heal(&mut hash, 4.0);
        //then takes 5 to be killed
        hash.insert(5, 5.0);

        // hash2 is the hypothitical total damage delt to the unit
        let hash2 = HashMap::from([(1, 2.0), (2, 2.0), (3, 2.0), (4, 2.0), (5, 5.0)]);

        // all the units should have a smaller share except for 5 who should have a larger share of the kill
        let kill_shared = calc_share(&hash);
        let percent_of_total_damage = calc_share(&hash2);
        for x in 1..5 {
            assert!(kill_shared.get(&x).unwrap() < percent_of_total_damage.get(&x).unwrap());
        }
        assert!(kill_shared.get(&5).unwrap() > percent_of_total_damage.get(&5).unwrap())
    }
}

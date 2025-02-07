use std::collections::HashMap;

use crate::Number;

type AgentId = usize;
/// in the ideal situation the portial og what we give a mate will be propotinal to what we invested
/// so we compare the actual share to the black slate to account for the change of the mates value
/// this also add response of not investing in investing in someone who we aren't effective in investing in
/// and also makes us more responsive to the reletive amount of attention given us as if they are giving more that the other then the share will get a boost which then couses us to invest more into them whou should couse us to give them more
pub fn compute_investment(
    values: &HashMap<AgentId, Number>,
    recieved: &HashMap<AgentId, Number>,
    given: &HashMap<AgentId, Number>,
) -> HashMap<AgentId, Number> {
    let investment_share: HashMap<AgentId, Number> = values
        .iter()
        .map(|(id, value)| {
            let this_given = given.get(id).cloned().unwrap_or(0.into());
            let this_recieved = recieved.get(id).cloned().unwrap_or(0.into());
            (*id, value + (this_given * this_recieved))
        })
        .collect();
    println!("{investment_share:?}");
    let total_shares: Number = investment_share.iter().map(|(_, x)| x).sum();
    investment_share
        .into_iter()
        .map(|(id, share)| (id, share / total_shares))
        .collect()
}
#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn feedback() {
        let values = HashMap::from([(0, 1.0), (1, 1.0), (2, 1.0), (3, 1.0)]);
        let recieved = HashMap::from([(0, 1.0), (1, 3.0), (2, 1.0), (3, 1.0)]);
        let given = HashMap::from([(0, 1.0), (1, 1.0), (2, 1.0), (3, 1.0)]);
        let given2: HashMap<AgentId, Number> = compute_investment(&values, &recieved, &given)
            .into_iter()
            .map(|(id, value)| (id, value * 10.0))
            .collect();

        println!(
            "1:{:?}",
            given.iter().collect::<BTreeMap<&AgentId, &Number>>()
        );

        println!(
            "2:{:?}",
            given2.iter().collect::<BTreeMap<&AgentId, &Number>>()
        );
        assert_eq!(
            given2,
            HashMap::from([(0, 2.0), (1, 4.0), (2, 2.0), (3, 2.0),])
        );
    }
}

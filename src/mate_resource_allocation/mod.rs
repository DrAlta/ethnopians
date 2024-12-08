use std::collections::HashMap;

use crate::Number;

type AgentId = usize;
/// in the ideal situation the portial og what we give a mate will be propotinal to what we invested
/// so we compare the actual share to the black slate to account for the change of the mates value
/// this also add response of not investing in investing in someone who we aren't effective in investing in
/// and also makes us more responsive to the reletive ammount of attention given us as if they are giving more that the other then the share will get a boost which then couses us to invest more into them whou should couse us to give them more
pub fn compute_investment(sensitivity: Number, values: &HashMap<AgentId, Number>, recieved: &HashMap<AgentId, Number>, given: &HashMap<AgentId, Number>) -> HashMap<AgentId, Number>{
    let total_value: Number = values.iter().map(|(_, x)|x).sum();

    let total_given: Number = given.iter().map(|(_, x)|x).sum();
    // We'' assume as if they don't have a value as a mate then we are uninterested in them so don't computer an invest me in them
    let investment_share: HashMap<AgentId, Number> = values.iter().map(|(id, value)|{
        let first = value / total_value;
        let this_given = given.get(id).cloned().unwrap_or(0.into());
        let second = this_given / total_given;
        let mult = second / first;
        let mult = ((mult * sensitivity) + (1.0-sensitivity)) / 2.0;
        let this_recieved = recieved.get(id).cloned().unwrap_or(0.into());
        if id == &1 {
            println!("first:{first}, second:{second}, mult:{mult}, given:{this_given}, revieved:{this_recieved}, {this_given} * {this_recieved} * {mult}:{}", this_given * this_recieved * mult);
        }
        (*id, this_given * this_recieved * mult)

    }).collect();
    println!("shares{investment_share:?}");
    let total_shares: Number = investment_share.iter().map(|(_, x)|x).sum();
    investment_share.into_iter().map(|(id, share)| {
        (id, share/total_shares)
    }).collect()
/*
    let first = mate_value / total_value_of_all_mates;
    let second = mate_give / total_of_all_mate_give;
    let mult = first / second;
    
    let investment = mate_give * mate_recieve * mult;
*/
}
#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn feedback(){
        let values = HashMap::from([
            (0, 1.0),
            (1, 1.0),
            (2, 1.0),
        ]);
        let recieved = HashMap::from([
            (0, 1.0),
            (1, 2.0),
            (2, 1.0),
        ]);
        let given = HashMap::from([
            (0, 1.0),
            (1, 1.0),
            (2, 1.0),
        ]);
        let given2 = compute_investment(1.0, &values, &recieved, &given);
        let recieved2 = HashMap::from([
            (0, 1.0),
            (1, 1.0),
            (2, 1.0),
        ]);
        let given3 = compute_investment(1.0, &values, &recieved2, &given2);
        let given4 = compute_investment(1.0, &values, &recieved2, &given3);
        println!("1:{:?}", given.iter().collect::<BTreeMap<&AgentId, &Number>>());

        println!("2:{:?}", given2.iter().collect::<BTreeMap<&AgentId, &Number>>());
        println!("3:{:?}", given3.iter().collect::<BTreeMap<&AgentId, &Number>>());
        println!("4:{:?}", given4.iter().collect::<BTreeMap<&AgentId, &Number>>());
        assert_eq!(given3, HashMap::new());
    }
}
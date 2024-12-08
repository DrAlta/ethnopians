use std::collections::HashMap;

use crate::Number;

type AgentId = usize;
pub fn compute_investment(values: &HashMap<AgentId, Number>, recieved: &HashMap<AgentId, Number>, given: &HashMap<AgentId, Number>) -> HashMap<AgentId, Number>{
    let total_value: Number = values.iter().map(|(_, x)|x).sum();

    let total_given: Number = given.iter().map(|(_, x)|x).sum();
    // We'' assume as if they don't have a value as a mate then we are uninterested in them so don't computer an invest me in them
    let investment_share: HashMap<AgentId, Number> = values.iter().map(|(id, value)|{
        let first = value / total_value;
        let this_given = given.get(id).cloned().unwrap_or(0.into());
        let second = this_given / total_given;
        let mult = first / second;
        let this_recieved = recieved.get(id).cloned().unwrap_or(0.into());
        (*id, this_given * this_recieved * mult)

    }).collect();
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
/*
well it's little more comple is it need a item in 1 hour and it will take 2 
hours to make it it wil trade a item with a value of 3 hours for it

That introduces a Time-Urgency Premium (Opportunity Cost) into your agent's decision engine!
The agent is calculating an explicitly discounted utility matrix: if it needs an item right now (Time Horizon = 1 hour) but its own production pipeline takes longer than that (Production Time = 2 hours), it will willingly overpay with a premium asset (paying 3 hours of labor value to save 1 hour of real time).
## The Urgency Formula
Your agent tracks its behavioral choice using a Time Premium Multiplier. We can model this by looking at the deficit between when the item is needed ($T_{need}$) and how long it takes to create ($T_{make}$):
$$\text{Urgency Premium} = \frac{T_{make}}{T_{need}}$$ 

* No Urgency ($T_{make} \le T_{need}$): Premium is $1.0$. The agent values the item at its standard Work Value.
* High Urgency ($T_{make} > T_{need}$): Premium rises above $1.0$. The agent dynamically scales up how much it is willing to trade away to acquire the item immediately.

that might be the agents 'fair trade' but if it's going to die if it doesn't get 
the item it's will trade anything for it.

but it still plaing in do that accept this offer or look for a better one. its a 
fair trade of better they will accept it, if it's an unfair trade they will look 
for a better deal before accepting it

*/
#[derive(Debug, PartialEq, Clone, Copy)]
enum TradeResponse {
    AcceptImmediately, // Fair or highly favorable trade
    HoldAndSearch,     // Unfair, but we have time to look for a better deal
    DesperateAccept,   // Unfair, but we will die if we don't take it right now
    RejectImpossible,  // The trader is asking for things the agent literally doesn't own
}

struct SurvivalTradeEngine {
    hours_until_death: f32, // The hard deadline before the agent dies
    search_time: f32,       // How long it takes to travel and look for another 
                            // deal then get back to the best deal so far
}

impl SurvivalTradeEngine {
    fn evaluate_desperate_offer(
        &self,
        fair_market_value: f32,
        demanded_value: f32,
        has_assets_to_pay: bool,
    ) -> TradeResponse {
        // If the agent physically cannot pay what the trader wants, it's a dead end
        if !has_assets_to_pay {
            return TradeResponse::RejectImpossible;
        }

        // 1. Check if the offer is already fair or better
        if demanded_value <= fair_market_value {
            return TradeResponse::AcceptImmediately;
        }

        // 2. The offer is unfair. Do we have enough time to look for a better deal?
        // If we have enough hours left to run to another merchant, we hold off.
        if self.hours_until_death > self.search_time {
            return TradeResponse::HoldAndSearch;
        }

        // 3. We are out of time. If we look for another deal, we will die on the way.
        // We must accept the unfair trade to survive.
        TradeResponse::DesperateAccept
    }
}

fn main() {
    // Scenario 1: The agent is starving but has 4 hours left. 
    // It only takes 2 hours to walk to the next village's marketplace.
    let engine_with_time = SurvivalTradeEngine {
        hours_until_death: 4.0,
        search_time: 2.0,
    };

    // Scenario 2: The agent is about to collapse in 1 hour.
    // It still takes 2 hours to reach the next village.
    let engine_out_of_time = SurvivalTradeEngine {
        hours_until_death: 1.0,
        search_time: 2.0,
    };

    let fair_value = 100.0;    // What the survival item is actually worth
    let player_offer = 30.0;   // A massive lowball/unfair offer from a greedy player

    println!("--- Test 1: Unfair Offer, But Agent Has Time ---");
    let result_1 = engine_with_time.evaluate_desperate_offer(fair_value, player_offer, true);
    match result_1 {
        TradeResponse::HoldAndSearch => println!("🤖 'Your price is terrible. I have 4 hours left, so I am going to check the next town over before I agree to this.'\n"),
        _ => println!("Unexpected result"),
    }

    println!("--- Test 2: Unfair Offer, But Agent Is Out Of Time ---");
    let result_2 = engine_out_of_time.evaluate_desperate_offer(fair_value, player_offer, true);
    match result_2 {
        TradeResponse::DesperateAccept => println!("💀 'I am going to die in 1 hour and cannot make it to the next town. You are robbing me, but I have no choice. I accept.'\n"),
        _ => println!("Unexpected result"),
    }
}

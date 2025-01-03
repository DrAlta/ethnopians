//!  Customer only buys 1 item per day
//! customer buys when `if price <= self.current_purchase_price {self.current_purchase_price = price}'
//! 
//! at end of day if a merchant has made a sale is increases selling_price and investing_tendancy else they derease them
//!  and pays a tax
//! day end when every customer has made a purache or visited ever merchane without finding a satify price 
//! 
//! customer leave is they don't make a purchace in a number of days
//! 
//! customer decrease ther current price if they haven't made a purchanse
//! 
//! merchch leave when they run out of money
//! ----
//! merchs will lower ther selling price at a rate to that the day they will go bankrupt they will be at ther production price
//! we have starting price, time to reach target price and target price, we want to find the constant acreation that starte a o volocity will take of rom starting price to target price in ther duration given
//! ?inver this 'gravity for when increasing the selling price'?
//! 
//! * ?use average purached per day?
//! ** ?how?
//! *** if average customer pr day is increasing increase prices
//! *** if aver customers per day is decreasing decrease prices
//! ***use a threashold. like if the number of customers was diffrent from predicted by more the N customers the increase/decrease prices
//! 
use thats_so_random::Pcg32;
pub struct Merchant {
    gold: Number,
    selling_price: Number,
    investing_tentancy: i16
}

impl Merchant {
    pub fn new(production_cost: Number, max_production_cost: Number, rng: &mut Pcg32) -> Self {
        Merchant {
            gold: 100000.0,
            selling_price: rng.random_range(production_cost, max_production_cost),
            investing_tentancy: 1,
        }
    }
}

pub struct Customer {
    max_purchase_price: Number,
    current_purchase_price: Number
}

impl Customer{
    pub fn new( rng: &mut Pcg32) -> Self {
        let max_purchase_price = rng.random_range(25.0, 400.0);
        Customer {
            current_purchase_price: max_purchase_price * 0.5,
            max_purchase_price,
        }
    }
}


type Number =f64;

pub fn kelly_criterion(probability_of_winning: Number, fraction_gain_on_win: Number, fraction_lost_on_lose: Number) -> Number {
    let probability_of_loosing = 1.0 - probability_of_winning;
    let fraction_to_bet = (probability_of_winning / fraction_lost_on_lose)- ( probability_of_loosing / fraction_gain_on_win);
    fraction_to_bet
}

pub fn kelly_simple(probability_of_winning: Number, fraction_gain_on_win: Number) -> Number {
    probability_of_winning - ((1.0 - probability_of_winning)/fraction_gain_on_win)
}

fn main(){
    let fraction_gain_on_win = 1.0;   
    let fraction_lost_on_lose = 1.0;
    let probability_of_winning = 0.6;
    
    assert_eq!(
        kelly_simple(probability_of_winning, fraction_gain_on_win),
        kelly_criterion(probability_of_winning, fraction_gain_on_win, fraction_lost_on_lose)
    );
    
}
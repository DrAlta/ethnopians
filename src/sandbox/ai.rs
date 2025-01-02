//! for the test aI just have it plant vegtibles in a field and harvest them ehen they are mature then replant them
//! if out od seed find neared plant to collect seeds from 
//! i thing a veg can be split into 3 seeds
//! useing hands on a plant produces vegs and consumes the plant
//! use an knife of a veg produces 3 seeds and consumes the veg
//! 
//! use a stone on stone produces a knife and consomes one stone
//! 
//! useinga knife on stick or visvera produces a axe and consumes the knife and stick
//! 
//! knife has higher DPS than axe but shorter range
//! 
//! 
//! seq{
//!     sat hunger
//!     sat sleep
//!     have house
//!     have_garden
//!     harvet_vag
//!     plant_vegs
//! }
//! ----
//! have_2_stone_2
//! ----
//! sel{
//!     invotory have >= 2 stone
//!     seq{
//!         go to stone
//!         take stone
//!     }
//! }
//! ----
//! have_2_stone
//! ----
//! seq {
//!     have_2_stone_2
//!     have_2_stone_2
//! }
//! ---
//!  have_knife
//! ----
//! sel{
//!     inevtor have >= 1 knife
//!     seq {
//!          have_2_stone
//!          combine stone and stone
//!     }
//! 
//! ---
//!  have_stick
//! ---
//! sel{
//!     invitory have >= 1 stick
//!     seq{
//!          go to tree
//!          use hands on tree
//!     }
//!
//! ---
//! have_axe
//! ---
//! sel {
//!  invtory have >= 1 axe
//!  seq {
//!      have_knife
//!      have_stick
//!      combine stick and knife
//!  }
//! ----
//! have_2_wood_2
//! ----
//! sel{
//!     invetory has >= 2 wood
//!     have_axe
//!     go_to_tree
//!     use axe on tree
//! }
//! ----
//! have_2_wood
//! ----
//! seq{
//!     have_2_wood_2
//!     have_2_wood_2
//! }
//! 
//! 
//! ---
//! have house
//! ---
//! 
//! Sel{
//!     is house in range
//!     seq {
//!         have_2_wood
//!         combine wood and wood
//!     }
//! ---
//! sat hunger
//! ---
//! selector{
//!     don't need to eat
//!     seq{
//!         selector{
//!             does he have veg
//!             get_veg
//!         }
//!         eat veg
//!     }
//! }
//! =====================market stuf below
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

pub struct Merchant {
    gold: Number,
    selling_price: Number,
    investing_tentancy: i16
}

impl Merchant {
    pub fn new() -> Self {
        Merchant {
            gold: 100000.0,
            selling_price: rng.rand_range(production_cost, max_production_cost),
            investing_tentancy: 
        }
    }
}

pub struct Customer {
    max_purchase_price: Number,
    current_purchase_price: Number
}

impl Customer{
    pub fn new() -> Self {
        let max_purchase_price = rng.rand_range(25.0, 400.0);
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
        foo(probability_of_winning, fraction_gain_on_win),
        kelly_criterion(probability_of_winning, fraction_gain_on_win, fraction_lost_on_lose)
    );
    
}
use crate::Number;


pub fn kelly_criterion(probability_of_winning: Number, fraction_gain_on_win: Number, fraction_lost_on_lose: Number) -> Number {
    let probability_of_loosing = Number::ONE - probability_of_winning;
    let fraction_to_bet = (probability_of_winning / fraction_lost_on_lose)- ( probability_of_loosing / fraction_gain_on_win);
    fraction_to_bet
}

pub fn kelly_simple(probability_of_winning: Number, fraction_gain_on_win: Number) -> Number {
    probability_of_winning - ((Number::ONE - probability_of_winning)/fraction_gain_on_win)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main(){
        let fraction_gain_on_win = Number::ONE;
        let fraction_lost_on_lose = Number::ONE;
        let probability_of_winning = Number::new(6, 10);
        
        assert_eq!(
            kelly_simple(probability_of_winning, fraction_gain_on_win),
            kelly_criterion(probability_of_winning, fraction_gain_on_win, fraction_lost_on_lose)
        );
        
    }
}
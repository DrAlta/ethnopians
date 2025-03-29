use crate::Number;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Agent {
    pub value: Number,
    pub threshold: Number,
    pub current_choices_value: Number,
}
impl Agent {
    pub fn date_desirability_of_switching(&mut self, date: &Agent) -> Number {
        let threshold = self.threshold.max(self.current_choices_value);
        match date.value.total_cmp(&threshold) {
            std::cmp::Ordering::Less => {
                let delta = date.value - threshold;
                println!("threshold:{threshold} date:{}, delta:{delta}", date.value);
                delta / threshold
            }
            std::cmp::Ordering::Equal => Number::ZERO,
            std::cmp::Ordering::Greater => {
                let delta = date.value - threshold;
                println!("delta:{delta}");
                date.value / delta
            }
        }
    }
    pub fn update_threshold(&mut self, rejected: bool, date: &Agent) {
        let delta = (date.value - self.threshold).abs();

        let step = delta * 0.05;
        println!("delta:{delta} step:{step}");

        if rejected {
            self.threshold -= step;
        } else {
            self.threshold += step;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn agents() -> (Agent, Agent) {
        (
            Agent {
                value: Number::TEN,
                threshold: Number::FIVE,
                current_choices_value: Number::TEN,
            },
            Agent {
                value: Number::TEN,
                threshold: Number::FIVE,
                current_choices_value: Number::TEN,
            },
        )
    }

    #[test]
    fn reject_test() {
        let (mut a, date) = agents();
        a.update_threshold(true, &date);
        assert_eq!(
            a,
            Agent {
                value: Number::TEN,
                threshold: Into::<Number>::into(4.75_f32),
                current_choices_value: Number::TEN,
            }
        );
    }
    #[test]
    fn twict_threshold_desirability_test() {
        let (mut a, date) = agents();
        assert_eq!(a.date_desirability_of_switching(&date), 0.0)
    }
    #[test]
    fn at_threshold_desirability_test() {
        let (mut a, _) = agents();
        let date = Agent {
            value: Number::TEN,
            threshold: Number::FIVE,
            current_choices_value: Number::TEN,
        };
        assert_eq!(a.date_desirability_of_switching(&date), 0.0)
    }
    #[test]
    fn half_threshold_desirability_test() {
        let (mut a, _) = agents();
        let date = Agent {
            value: Number::FIVE,
            threshold: Number::FIVE,
            current_choices_value: Number::TEN,
        };
        assert_eq!(a.date_desirability_of_switching(&date), -0.5)
    }
    #[test]
    fn zero_value_desirability_test() {
        let (mut a, _) = agents();
        let date = Agent {
            value: Number::ZERO,
            threshold: Number::FIVE,
            current_choices_value: Number::TEN,
        };
        assert_eq!(a.date_desirability_of_switching(&date), -1.0)
    }
}

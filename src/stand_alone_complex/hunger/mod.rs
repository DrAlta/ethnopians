pub struct HungerTracker {
    /// Has the player eaten their first bite today?
    pub whetted: bool,
    /// Calories burned since waking up
    pub cal_burned: f32,
    /// Sweetness penalty tracker (increases with artificial sweetness)
    pub sweetness_score: f32,
    /// Basal Metabolic Rate (calories burned naturally per day)
    pub bmr: f32,
    /// Tuning constant 'k' to adjust the steepness of the hunger curve
    pub curve_steepness_k: f32,
}

impl HungerTracker {
    /// Creates a new day tracker. Hunger initializes at 0.
    pub fn new(bmr: f32, curve_steepness_k: f32) -> Self {
        Self {
            whetted: false,
            cal_burned: 0.0,
            sweetness_score: 0.0,
            bmr,
            curve_steepness_k,
        }
    }

    /// Resets variables when the player goes to sleep.
    pub fn reset_on_sleep(&mut self) {
        self.whetted = false;
        self.cal_burned = 0.0;
        self.sweetness_score = 0.0;
    }

    /// Burns calories over time (e.g., ticking every game minute or hour).
    pub fn burn_calories(&mut self, amount: f32) {
        if amount > 0.0 {
            self.cal_burned += amount;
        }
    }

    /// Triggers when the player eats. 
    /// Real calories reduce the calorie deficit. Artificial sweetness adds a hunger penalty.
    pub fn eat(&mut self, real_calories: f32, artificial_sweetness: f32) {
        // Taking any bite flips the hunger gate wide open
        self.whetted = true;

        // Absorbing real calories pulls down the accumulated calorie burn deficit
        self.cal_burned = (self.cal_burned - real_calories).max(0.0);

        // Track how much sweetness has been introduced
        self.sweetness_score += artificial_sweetness;
    }

    /// Calculates the current hunger level using your exact equations.
    pub fn current_hunger(&self) -> f32 {
        if !self.whetted {
            return 0.0;
        }

        // base_hunger = (x / (k + x)) * sqrt(x)
        let x = self.cal_burned;
        let base_hunger = if x > 0.0 {
            (x / (self.curve_steepness_k + x)) * x.sqrt()
        } else {
            0.0
        };

        // point_where_50percent_effective = BMR * (24 / 4) -> simplified to BMR * 6.0
        let point_50_percent = self.bmr * 6.0;

        // sweetness_hunger = sweetness * 0.5.powf(cal / point_50_percent)
        let sweetness_hunger = if point_50_percent > 0.0 {
            self.sweetness_score * 0.5f32.powf(x / point_50_percent)
        } else {
            0.0
        };

        base_hunger + sweetness_hunger
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoveltyRating {
    PerfectNew = 0, // 100% Brand new / deeply moving
    Good = 1,       // Enjoyable, but initial wow factor wearing off
    Passing = 2,    // Neutral / starting to get used to it
    Overplayed = 3, // Actively getting bored
    BurntOut = 4,   // Never want to see/hear this again for a while
}

#[derive(Debug, Clone)]
pub struct NoveltyScheduler {
    pub saturation_index: f64,   // Inherent burnout rate
    pub burnout_resistance: f64, // How fast it burns out (lower = faster)
    pub last_fatigue: f64,       // Scale 0.0 to 1.0 (0.0 = completely fresh)
}

impl NoveltyScheduler {
    /// Creates a tracker for a brand new item.
    pub fn new() -> Self {
        Self {
            saturation_index: 2.0,
            burnout_resistance: 1.0,
            last_fatigue: 0.0,
        }
    }

    /// Calculates how much the item has refreshed over a given number of days.
    pub fn get_current_fatigue(&self, days_passed: f64) -> f64 {
        let recovery_halflife = 5.0 * self.burnout_resistance;
        let current_fatigue = self.last_fatigue * 0.5f64.powf(days_passed / recovery_halflife);

        // Clamp between 0.0 and 1.0 to prevent floating-point anomalies
        current_fatigue.clamp(0.0, 1.0)
    }

    /// Updates the item's profile when you experience it again.
    pub fn record_experience(&mut self, days_passed: f64, rating: NoveltyRating) {
        // 1. Calculate fatigue right before this new experience
        let starting_fatigue = self.get_current_fatigue(days_passed);

        // 2. Map the enum rating to a mathematical fatigue impact (0.0 to 1.0)
        let rating_val = rating as i32 as f64;
        let rating_impact = rating_val / 4.0;

        // Cap maximum immediate fatigue at 100% (1.0)
        self.last_fatigue = (starting_fatigue + rating_impact).min(1.0);

        // 3. Update Burnout Resistance based on user preference
        match rating {
            NoveltyRating::Overplayed | NoveltyRating::BurntOut => {
                // If you are sick of it, it burns out much faster next time
                self.burnout_resistance = (self.burnout_resistance * 0.7).max(0.1);
            }
            NoveltyRating::PerfectNew => {
                // If it felt completely fresh, its resistance recovers
                self.burnout_resistance = (self.burnout_resistance * 1.2).min(5.0);
            }
            _ => {} // Passing and Good keep resistance stable
        }
    }

    /// Predicts how many days to wait until fatigue drops to your target percentage.
    pub fn days_to_fresh(&self, target_percent: f64) -> f64 {
        let target = target_percent / 100.0;
        if self.last_fatigue <= target {
            return 0.0;
        }

        let recovery_halflife = 5.0 * self.burnout_resistance;
        // Half-life decay math: days = halflife * log0.5(target / current_fatigue)
        let days = recovery_halflife * (target / self.last_fatigue).log(0.5);
        days.max(0.0)
    }
}

fn main() {
    let mut tracker = NoveltyScheduler::new();

    println!("--- Simulation: Overplaying a Song ---");

    // Day 0: Listen to it when it's kinda fine but getting old
    println!("\n[Day 0] Listening to the song...");
    tracker.record_experience(0.0, NoveltyRating::Passing);
    println!("Fatigue level: {:.1}%", tracker.last_fatigue * 100.0);
    println!(
        "Days until 10% fatigue: {:.1} days",
        tracker.days_to_fresh(10.0)
    );

    // Day 1: Listen again, now you're tired of it
    println!("\n[Day 1] Listening again too soon...");
    tracker.record_experience(1.0, NoveltyRating::Overplayed);
    println!("Fatigue level: {:.1}%", tracker.last_fatigue * 100.0);
    println!(
        "Days until 10% fatigue: {:.1} days",
        tracker.days_to_fresh(10.0)
    );

    // Day 2: One more time, total burnout
    println!("\n[Day 2] Listened to it on repeat. Max Burnout!");
    tracker.record_experience(1.0, NoveltyRating::BurntOut);
    println!("Fatigue level: {:.1}%", tracker.last_fatigue * 100.0);

    // Check how long we have to wait for the track to feel fresh again
    let wait_days = tracker.days_to_fresh(10.0);
    println!("Days until 10% fatigue: {:.1} days", wait_days);
}

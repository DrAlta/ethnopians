use super::super::{AROMATICS_SIZE, Customer, FlavorVector, TECHNICAL_SIZE, calculate_balance, score_imbalance};


impl Customer {
    pub fn critique_dish(&self, fv: &FlavorVector) -> f32 {
        let balance_scores = calculate_balance(fv);
        let imbalance_score = score_imbalance(&balance_scores);
        let total_intensity: f32 = balance_scores.iter().sum();

        // Base: Balance minus "Blandness" error
        let mut opinion = -( imbalance_score + (total_intensity - self.target_intensity).abs());
        {
            let avg_intensity = total_intensity / TECHNICAL_SIZE as f32;

        for i in 0..TECHNICAL_SIZE {
            let actual_offset = balance_scores[i] - avg_intensity;
            
            // Relative Preference Logic
            let bias_error = (actual_offset - self.technical_ideal_relative_offsets[i]).abs();
            
            // Extreme Preference Logic
            let extreme_bonus = fv.technical[i] * self.technical_extreme_prefs[i];

            opinion += extreme_bonus - bias_error;
        }
        }
        {
        // aromatics score
            let avg = fv.aromatics.iter().sum::<f32>() / AROMATICS_SIZE as f32;
            for i in 0..AROMATICS_SIZE {
                let actual_offset = balance_scores[i] - avg;
                
                // Relative Preference Logic
                let bias_error = (actual_offset - self.aromatics_ideal_relative_offsets[i]).abs();
                
                // Extreme Preference Logic
                let extreme_bonus = fv.aromatics[i] * self.aromatics_extreme_prefs[i];

                opinion += extreme_bonus - bias_error;
            }

        }
        opinion
    }
}

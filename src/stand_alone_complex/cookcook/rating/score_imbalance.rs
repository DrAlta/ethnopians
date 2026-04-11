pub fn score_imbalance(fv: &[f32;7]) -> f32 {
    let avg: f32 = fv.iter().sum::<f32>() / 7.0;
    let mut variance_sum: f32 = 0.0;
    
    // Only check balance for the first 7 (The core "Equation")
    for i in 0..7 {
        variance_sum += (fv[i] - avg).abs();
    }
    
    variance_sum / 7.0
}

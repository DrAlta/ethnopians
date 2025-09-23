use crate::Number;

/// Calculates the lower bound of the Wilson Score Interval.
/// This is a common way to rank items based on upvotes and downvotes,
/// especially when dealing with varying numbers of votes.
///
/// # Arguments
///
/// * `upvotes` - The number of positive votes (successes).
/// * `downvotes` - The number of negative votes (failures).
/// * `confidence_level` - The desired confidence level (e.g., 0.95 for 95% confidence).
///   A z-score of 1.96 is typically used for a 95% confidence level.
///
/// # Returns
///
/// The lower bound of the Wilson Score Interval, representing a conservative
/// estimate of the true proportion of positive votes. Returns 0.0 if there are no votes.
///
/// # Examples
///
/// ```
/// let upvotes = 100;
/// let downvotes = 40;
/// let score = calculate_wilson_score(upvotes, downvotes);
/// println!("Wilson Score: {}", score);
/// ```
pub fn calculate_wilson_score(upvotes: u64, downvotes: u64) -> Number {
    let n = Into::<Number>::into(upvotes + downvotes);

    // If no votes, return 0 to avoid division by zero and represent an unknown ranking.
    if n == Number::ZERO {
        return Number::ZERO;
    }

    let p = Into::<Number>::into(upvotes) / n;

    // For a 95% confidence interval, z-score is approximately 1.96.
    // For simplicity, we'll use a fixed z-score here. In a more robust
    // implementation, you'd calculate this based on the confidence_level
    // using a statistical library or approximation.
    let z = Into::<Number>::into(1.96); 

    let numerator = p + (z * z) / (Number::TWO * n) - error_bound(z, p, n);
    let denominator = Number::ONE + (z * z) / n;

    numerator / denominator
}

fn error_bound(z: Number, p: Number, n: Number) -> Number {
    z * ((p * (Number::ONE - p) + (z * z) / (Number::FOUR * n)) / n).sqrt()

}
pub fn calculate_error_bound(upvotes: u64, downvotes: u64)-> Option<Number>{
    let n = Into::<Number>::into(upvotes + downvotes);

    // If no votes, return 0 to avoid division by zero and represent an unknown ranking.
    if n == Number::ZERO {
        return None;
    }

    let p = Into::<Number>::into(upvotes) / n;

    // For a 95% confidence interval, z-score is approximately 1.96.
    // For simplicity, we'll use a fixed z-score here. In a more robust
    // implementation, you'd calculate this based on the confidence_level
    // using a statistical library or approximation.
    let z = Into::<Number>::into(1.96);
    Some(error_bound(z, p, n))
}
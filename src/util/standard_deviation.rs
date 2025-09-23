use crate::Number;

pub fn standard_deviation(data: &[Number]) -> Option<Number> {
    let count = data.len();

    if count == 0 {
        return None; // Cannot calculate std dev for an empty dataset
    }

    // Calculate the mean
    let mean = data.iter().sum::<Number>() / Into::<Number>::into(count);

    // Calculate the variance (average of squared differences from the mean)
    let variance = data.iter()
        .map(|&value| {
            let diff = mean - value;
            diff * diff
        })
        .sum::<Number>() / Into::<Number>::into(count); // Use count for population std dev, count-1 for sample

    // The standard deviation is the square root of the variance
    Some(variance.sqrt())
}
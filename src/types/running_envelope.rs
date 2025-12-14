use crate::{Number, IOTA};
/// A structure for tracking the running envelope of a signal.
///
/// This struct maintains smoothed estimates for the overall signal's:
/// - Running average (`avg`)
/// - Smoothed maximum value (`max_val`) and a further smoothed upper envelope (`high`)
/// - Smoothed minimum value (`min_val`) and a further smoothed lower envelope (`low`)
///
/// It uses exponential smoothing for both tracking the signal’s level and its dynamic range.
/// The `attack` and `release` parameters control how quickly the envelope tracks upward or downward deviations,
/// while `decay` determines the overall smoothing factor applied when updating these values.
pub struct RunningEnvelope {
    /// The smoothed maximum envelope value.
    ///
    /// This value tracks the high extremes of the signal using a hybrid method
    /// that considers both the instantaneous `new_sample` and the running
    /// average.
    pub fast_attack_high: Number,

    /// The further smoothed high envelope value.
    ///
    /// This field is a filtered version of `fast_attack_high` designed to
    /// provide a stable upper bound that reacts less abruptly to transient
    /// spikes.
    pub high: Number,

    /// The running average of the signal.
    ///
    /// This is computed as an exponential moving average of the input values
    /// and provides a baseline reference level for the signal.
    pub avg: Number,

    /// The further smoothed low envelope value.
    ///
    /// This field is a filtered version of `fast_attack_low` meant to yield a
    /// stable lower bound, prMessageing overly reactive changes due to brief
    /// dips.
    pub low: Number,

    /// The smoothed minimum envelope value.
    ///
    /// This value tracks the low extremes of the signal and is updated
    /// similarly to `fast_attack_high`, but for downward deviations.
    pub fast_attack_low: Number,

    /// Release parameters.
    ///
    /// This parameter governs how quickly the maximun envelope reacts to
    /// decreases in the signal.
    /// A higher value means the envelope will drop more rapidly when the
    /// signal stops raising as high.
    pub max_release: Number,

    /// This parameter governs how quickly the minimum envelope reacts to
    /// increases in the signal.
    /// A higher value means the envelope will raise more rapidly when the
    /// signal stops droping as low.
    pub min_release: Number,

    /// Decay (smoothing) factor.
    ///
    /// This is used in the exponential smoothing calculations for the
    /// running average and the envelope values. A higher decay value gives more
    /// weight to historical values, making the smoothing more pronounced.
    pub decay: Number,
}

impl RunningEnvelope {
    /// Updates the running envelope based on a new sample.
    ///
    /// This method updates:
    /// - The `avg` as an exponential moving average of the incoming `new_sample`.
    /// - The `max_val` and `min_val` envelopes using a hybrid update (comparing the new sample to the `avg`).
    /// - The further smoothed `high` and `low` envelopes based on `max_val` and `min_val` respectively.
    ///
    /// # Parameters
    /// - `new_sample`: The most recent sample from the signal.
    pub fn update(&mut self, new_sample: Number) -> Number {
        // Calculate the weight for previous values (e.g., if decay is 10, then weight_old = 9).
        let weight_old = Number::ONE - self.decay;

        // Update the running average.
        self.avg = (self.avg * weight_old) + (new_sample * self.decay);

        // For the max envelope, if the `new_sample` is geater that the old
        // value use it else do a running average with the larger of the
        // `new_sample` or the average
        self.fast_attack_high = if new_sample > self.fast_attack_high {
            new_sample
        } else {
            self.fast_attack_high * (Number::ONE - self.max_release)
                + (new_sample.max(self.avg) * self.max_release)
        };

        // Smooth the high envelope based on the max envelope.
        self.high = (self.high * weight_old) + (self.fast_attack_high * self.decay);

        // For the min envelope, if the `new_sample` is leaser that the old
        // value use it else do a running average with the leaser of the
        // `new_sample` or the average
        self.fast_attack_low = if new_sample < self.fast_attack_low {
            new_sample
        } else {
            self.fast_attack_low * (Number::ONE - self.min_release)
                + (new_sample.min(self.avg) * self.min_release)
        };

        // Smooth the low envelope based on the min envelope.
        self.low = (self.low * weight_old) + (self.fast_attack_low * self.decay);

        self.position(new_sample)
    }
    /// I had Chad do this bezier, my dylexia make reading Standard math notation difficult.
    ///
    /// Maps a value to a subjective rating in the range [-1, 1] using a quadratic Bézier curve.
    ///
    /// This mapping uses the envelope's low, average, and high:
    /// - When `value` is equal to `low`, the output is -1.
    /// - When `value` is equal to `avg`, the output is 0.
    /// - When `value` is equal to `high`, the output is 1.
    ///
    /// The method works as follows:
    /// 1. Normalize the input into a parameter t (0 at low, 1 at high).
    /// 2. Calculate t_avg = (avg - low) / (high - low).
    /// 3. Determine the control point in output space:
    ///      P₁ = (1 - 2*t_avg) / (2*t_avg*(1 - t_avg))
    /// 4. Return B(t) = (1-t)²*(-1) + 2*(1-t)*t*P₁ + t²*(1).
    ///
    /// If the envelope range is too small (to avoid division by zero),
    /// the function returns 0.0.
    pub fn position(&self, value: Number) -> Number {
        // Use the envelope's low and high as the endpoints.
        let range = self.high - self.low;
        if range.abs() < *IOTA {
            return Number::ZERO;
        }
        // Normalize the input value: t in [0,1]
        let t = ((value - self.low) / range).clamp(Number::ZERO, Number::ONE);
        // Normalize the average's location
        let t_avg = ((self.avg - self.low) / range).clamp(Number::ZERO, Number::ONE);

        // Avoid division-by-zero if avg is extremely close to low or high.
        if t_avg.abs() < *IOTA || (Number::ONE - t_avg).abs() < *IOTA {
            // Fall back to a simple linear mapping from -1 to 1.
            return Number::TWO * t - Number::ONE;
        }

        // Compute the control point P₁ so that B(t_avg) = 0.
        let p1 =
            (Number::ONE - Number::TWO * t_avg) / (Number::TWO * t_avg * (Number::ONE - t_avg));

        // Quadratic Bézier curve:
        // B(t) = (1-t)² * (-1) + 2*(1-t)*t * p1 + t² * (1)
        let subjective = (Number::ONE - t).powi(2) * (Number::NEG_ONE)
            + Number::TWO * (Number::ONE - t) * t * p1
            + t.powi(2) * Number::ONE;
        subjective
    }
    /*
    /// Computes the subjective position of a given value relative to the running envelope.
    ///
    /// The envelope is defined by the running average (`avg`) and the upper bound (`high`).
    /// This function returns a normalized value such that:
    /// - A `value` equal to `avg` yields 0.0.
    /// - A `value` equal to `high` yields 1.0.
    /// - Values greater than the average but below high linearly map between 0 and 1.
    /// - Values below the average yield negative positions.
    ///
    /// This is used for a hedonic treadmill to rate items on an absolute scale and then
    /// interpret them relative to a character's subjective baseline happiness.
        pub fn position(&self, value: Number) -> Number {
            // Calculate the effective range. Here we assume that self.high is the upper bound.
            // (You could similarly use self.avg - self.low for negative deviations if desired.)
            let range = self.high - self.avg;

            // If range is too close to zero, return 0.0 to avoid division errors.
            if range.abs() < IOTA {
                return 0.0;
            }

            // Determine how far the value is above (or below) the average.
            let offset = value - self.avg;

            // Normalize the offset by the range.
            offset / range
        }
    */
}

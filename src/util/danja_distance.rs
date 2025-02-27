type Number = f64;

/// this is an aproximate distance funtion
/// acturaly it's an upper bound on the distance
/// the greate the diffreance between the to the less te percant error
pub fn danja_distance(x: Number, y: Number) -> Number {
    let (max, min) = if y < x { (x, y) } else { (y, x) };
    let diff = max - min;
    let under = max + (min * (1.0 / max));

    // these where fitted to
    //      (1, 0),     (10, 0),    (100, 0),   (1000, 0),     (1, 1),    (10, 1),    (100, 1),   (1000, 1),   (10, 10), (100, 10),  (1000, 10),  (100, 100), (1000, 100), (1000, 1000) the error is:
    // -0.01342368  -0.13302948  -1.20956212  -0.02235638  0.49943064  0.29039857  -0.82390048  -0.01311637  0.49430644  2.2347317   0.06147975,  0.44306443  -0.02808723   -0.06935571
    let [min_diff_c, min_diff_m, max_diff_c, max_diff_m] = [
        -7.29186982e-02,
        -4.04831389e-04,
        -1.34370951e-02,
        1.34147387e-05,
    ];

    let min_correction = min * (min_diff_c + (diff * min_diff_m));
    let max_correction = max * (max_diff_c + (diff * max_diff_m));
    let correction = min_correction + max_correction;
    ((under + max + min) / 2.0) + correction
}
/*
fn main(){
    let x = 10_f64;
    let y = 10.0;
    println!(
    "{}\n{}",
    ((x*x)+(y*y)).sqrt(),
    danja_distance(x,y)
    )
}
*/

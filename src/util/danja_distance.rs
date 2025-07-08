type Number = f64;

/// this is an aproximate distance funtion
/// acturaly it's an upper bound on the distance
/// the greate the diffreance between the to the less te percant error
pub fn danja_distance(mut x: Number, mut y: Number) -> Number {
    x = x.abs();
    y = y.abs();
    let (max, min) = if y < x { (x, y) } else { (y, x) };
    let diff = max - min;
    let under = max + (min * (1.0 / max));

    // these where fitted to
    //      (1, 0),     (10, 0),    (100, 0),   (1000, 0),     (1, 1),    (10, 1),    (100, 1),   (1000, 1),   (10, 10), (100, 10),  (1000, 10),  (100, 100), (1000, 100), (1000, 1000) the error is:
    // -0.01342368  -0.13302948  -1.20956212  -0.02235638  0.49943064  0.29039857  -0.82390048  -0.01311637  0.49430644  2.2347317   0.06147975,  0.44306443  -0.02808723   -0.06935571
    let [min_diff_c, min_diff_m, max_diff_c, max_diff_m] = [
        -5.07106089e-01,
        -1.20884737e-04,
        1.14701372e-02,
        -1.15830904e-14,
    ];

    let min_correction = min * (min_diff_c + (diff * min_diff_m));
    let max_correction = max * (max_diff_c + (diff * max_diff_m));
    let correction = min_correction + max_correction;

    // run it thru a step of Newton's Method
    let guess = ((under + max + min) * 0.5) + correction;
    let sqrd = (x * x) + (y * y);
    0.5 * (guess + (sqrd / guess))
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

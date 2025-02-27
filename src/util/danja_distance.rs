type Number =f64;

/// this is an aproximate distance funtion
/// acturaly it's an upper bound on the distance
/// the greate the diffreance between the to the less te percant error
pub fn danja_distance(x:Number, y:Number) -> Number {
    let (max, min) = if y < x {(x,y)} else {(y,x)};
    let diff = max - min;
    let under = max + (min * (1.0 / max));

    // these where fitted to 
//          (1,1),        (10,10),      (100,100),        (1000,1),    (1000, 100),    (1000, 1000),      (1001, 1),     (1100, 100) the error is:
// 4.99450053e-01  4.94500531e-01  4.45005306e-01  -1.77946164e-02  1.97436080e-04  -4.99469412e-02  1.77768173e-02  -1.77692237e-04
    
    let [min_diff_c, min_diff_m, max_diff_c, max_diff_m] = [-5.03056589e-02, -4.04488225e-04, -3.60307257e-02, 3.60033238e-05];

    let min_correction = min * (min_diff_c + (diff * min_diff_m));
    let max_correction = max * (max_diff_c + (diff * max_diff_m));
    let correction = min_correction + max_correction;
    (( under + max + min) / 2.0 ) + correction
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
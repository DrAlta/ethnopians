type Number =f64;

/// this is an aproximate distance funtion
/// acturaly it's an upper bound on the distance
/// the greate the diffreance between the to the less te percant error
pub fn danja_distance(x:Number, y:Number) -> Number {
    let (max,min) = if y < x {(x,y)} else {(y,x)};
    let d = max - min;
    let q = max + (min * (1.0 / max));
    let l = 0.5 * (min * 0.020671430 * d);
    (( q + max + min) / 2.0 ) - l
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
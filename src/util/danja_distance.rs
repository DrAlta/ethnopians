type Number =f64;

/// this is an aproximate distance funtion
/// acturaly it's an upper bound on the distance
pub fn danja_distance(x:Number, y:Number) -> Number {
    let (s,b) = if y < x {(x,y)} else {(y,x)};
    let q = s + (b * (1.0 / s));
    let l = 0.5 * (b * 0.17160241048);
    (( q + s + b) / 2.0 ) - l
}
fn main(){
    let x = 10_f64;
    let y = 10.0;
    println!(
    "{}\n{}", 
    ((x*x)+(y*y)).sqrt(), 
    danja(x,y)
    )
}
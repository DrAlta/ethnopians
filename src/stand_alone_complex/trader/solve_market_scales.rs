/*
it's for a economicagent like emergent economs for RPG. the agent sbserves 
trades of a groupsa od items for each other. it need to derming what it would 
value the item under the assuetino that to two groups of items are equal value. 
it knows the value of some on the items and some of them on unknown. it starts 
with an assumption about he value of the unknown items then this fustion scales 
the unknown values so that the two sides of the trade are of equal value

no you scale one unknown up and scale the other on down

but if one side has a unknow valeu of 0 this returns none, when it could just 
adjust the value of the unknown of the other side so that that side matches the 
side with known value
*/
fn solve_market_scales(
    total_a_known: f32,
    total_a_unknown: f32,
    total_b_known: f32,
    total_b_unknown: f32,
) -> Option<[f32; 2]> {
    // Case 1: Neither side has any unknowns to adjust
    if total_a_unknown == 0.0 && total_b_unknown == 0.0 {
        return if total_a_known == total_b_known {
            Some([1.0, 1.0]) // Already perfectly balanced, scales don't matter
        } else {
            None // Knowns are unequal and no unknowns exist to balance them
        };
    }

    // Case 2: Only Side B has unknowns (Linear fallback)
    if total_a_unknown == 0.0 {
        let denominator = total_a_known - total_b_known;
        if denominator == 0.0 { return None; } // Prevent division by zero if Y_u exists but knowns match
        let s = total_b_unknown / denominator;
        return if s == 0.0 { None } else { Some([s, 1.0 / s]) };
    }

    // Case 3: Only Side A has unknowns (Linear fallback)
    if total_b_unknown == 0.0 {
        let s = (total_b_known - total_a_known) / total_a_unknown;
        return if s == 0.0 { None } else { Some([s, 1.0 / s]) };
    }

    // Case 4: Both sides have unknowns (Full Quadratic Formula)
    let a = total_a_unknown;
    let b = total_a_known - total_b_known;
    let c = -total_b_unknown;

    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        return None; // Real values cannot balance this specific transaction ratio
    }

    let sqrt_disc = discriminant.sqrt();
    let s1 = (-b + sqrt_disc) / (2.0 * a);
    let s2 = (-b - sqrt_disc) / (2.0 * a);

    // Prefer a positive economic scaling factor if available
    let s = if s1 > 0.0 { s1 } else { s2 };

    if s == 0.0 {
        None
    } else {
        Some([s, 1.0 / s])
    }
}

fn main() {
    // Testing the scenario where Side A has 0 unknown values
    let total_a_kn = 25.0;
    let total_a_unk = 0.0; // 0 unknowns here!
    let total_b_kn = 10.0;
    let total_b_unk = 30.0; 

    if let Some(x) = solve_market_scales(total_a_kn, total_a_unk, total_b_kn, total_b_unk) {
        let final_a = total_a_kn + (total_a_unk * x[0]);
        let final_b = total_b_kn + (total_b_unknown * x[1]);
        
        println!("Scale Factor A (s): {:.6}", x[0]);
        println!("Scale Factor B (1/s): {:.6}", x[1]);
        println!("Side A: {total_a_kn} + ({total_a_unk} * {:.6}) = {:.6}", x[0], final_a);
        println!("Side B: {total_b_kn} + ({total_b_unk} * {:.6}) = {:.6}", x[1], final_b);
        println!("Balanced Successfully: {:.6} = {:.6}", final_a, final_b);
    } else {
        println!("Could not balance the trade.");
    }
}

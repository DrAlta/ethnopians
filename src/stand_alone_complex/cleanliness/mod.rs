use std::collections::{HashMap, BTreeSet};

// Define the Enum as specified
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Types {
    TypeA,
    TypeB,
    TypeC,
    // ... potentially more types
}

/// Helper function to calculate the total effective space occupied by the current mixture.
/// Uses the provided logic of summing all amounts, then subtracting total saved space from compatibility.
fn calculate_total_effective_space(
    mixture: &HashMap<Types, f32>,
    compatibility: &HashMap<BTreeSet<Types>, f32>
) -> f32 {
    let total_sum: f32 = mixture.values().sum();

    // Iterate over the compatibility HashMap and sum up all the 'space saved' for existing pairs
    let total_saved_space: f32 = compatibility.iter().filter_map(
        |(pair_types, compatibility_value)| {
            // Check if both types in the pair exist in the current mixture
            let amounts: Vec<&f32> = pair_types.iter()
                .filter_map(|t| mixture.get(t))
                .collect();

            // If we have both amounts (length of amounts is 2), calculate saved space for this pair
            if amounts.len() == 2 {
                let min_amount = amounts[0].min(*amounts[1]);
                // Saved space = min * (1 - compatibility)
                let space_saved_for_pair = min_amount * (1.0 - compatibility_value);
                Some(space_saved_for_pair)
            } else {
                // If one or both types are not in the mixture, no space is saved for this pair
                None
            }
        }
    ).sum();

    let effective_space = total_sum - total_saved_space;
    // Ensure the result is non-negative
    effective_space.max(0.0)
}


/// Adds a specified amount of a type to the mixture, scaling down all types proportionally 
/// if the total effective space exceeds 100 units.
///
/// This version accepts that the final amount of `tyep` will be scaled proportionally with others
/// if the limit is exceeded, rather than remaining fixed at `initial + amount`.
pub fn add_to_mixture(
    tyep: Types,
    amount: f32,
    mixture: &mut HashMap<Types, f32>,
    compatibility: &HashMap<BTreeSet<Types>, f32>
) -> Result<(), String> {
    const LIMIT: f32 = 100.0;

    // ---1. Calculate the potential total effective space *if* we simply add the new amount ---
    // We need to simulate adding the new amount to the mixture temporarily to use the calculation helper function
    let initial_amount = *mixture.get(&tyep).unwrap_or(&0.0);
    let amount_after_add_temp = initial_amount + amount;
    
    // Temporarily update the mixture for the calculation
    mixture.insert(tyep, amount_after_add_temp);

    let effective_space_after_add = calculate_total_effective_space(mixture, compatibility);

    // --- 2. Check if scaling is needed ---
    if effective_space_after_add > LIMIT {
        // Calculate the proportional scaling multiplier
        let mult = LIMIT / effective_space_after_add;
        
        // Scale *all* values in the mixture proportionally
        for v in mixture.values_mut() {
            *v *= mult;
        }
    } 
    
    // The mixture is now in its final state. The `insert` call updated the target type.
    // If no scaling occurred, the `amount_after_add_temp` remains. 
    // If scaling occurred, all amounts were scaled by `mult`.

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_compatibility() -> HashMap<BTreeSet<Types>, f32> {
        let mut compatibility = HashMap::new();
        // A/B compatibility: 0.25 (25% overlap/saved space)
        let mut pair_ab = BTreeSet::new();
        pair_ab.insert(Types::TypeA);
        pair_ab.insert(Types::TypeB);
        compatibility.insert(pair_ab, 0.25); 

        // B/C compatibility: 0.10 (10% overlap/saved space)
        let mut pair_bc = BTreeSet::new();
        pair_bc.insert(Types::TypeB);
        pair_bc.insert(Types::TypeC);
        compatibility.insert(pair_bc, 0.10); 

        // A/C compatibility: 0.0 (No overlap/saved space, they are opposed)
        let mut pair_ac = BTreeSet::new();
        pair_ac.insert(Types::TypeA);
        pair_ac.insert(Types::TypeC);
        compatibility.insert(pair_ac, 0.0); 

        compatibility
    }

    #[test]
    fn test_space_calculation_user_logic() {
        let compatibility = setup_compatibility();
        let mut mixture = HashMap::new();

        // Test user's example: 50 TypeA, 50 TypeB, 0.25 compatibility
        mixture.insert(Types::TypeA, 50.0);
        mixture.insert(Types::TypeB, 50.0);

        // Sum = 100. Saved space = min(50, 50) * (1 - 0.25) = 50 * 0.75 = 37.5
        // Effective space = 100 - 37.5 = 62.5
        let total_space = calculate_total_effective_space(&mixture, &compatibility);
        assert!((total_space - 62.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_add_without_scaling() {
        let compatibility = setup_compatibility();
        let mut mixture = HashMap::new();
        
        // Start with 50 units of TypeA
        add_to_mixture(Types::TypeA, 50.0, &mut mixture, &compatibility).unwrap();
        assert!((mixture[&Types::TypeA] - 50.0).abs() < f32::EPSILON);
        let space = calculate_total_effective_space(&mixture, &compatibility);
        assert!((space - 50.0).abs() < f32::EPSILON);

        // Add 10 units of TypeB (Total effective space becomes 50+10 - (10 * 0.75) = 52.5, which is < 100)
        add_to_mixture(Types::TypeB, 10.0, &mut mixture, &compatibility).unwrap();
        assert!((mixture[&Types::TypeA] - 50.0).abs() < f32::EPSILON);
        assert!((mixture[&Types::TypeB] - 10.0).abs() < f32::EPSILON);
        let space_after = calculate_total_effective_space(&mixture, &compatibility);
        assert!((space_after - 52.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_add_with_scaling() {
        let compatibility = setup_compatibility();
        let mut mixture = HashMap::new();

        // Start with 80 units of TypeA (Space: 80)
        add_to_mixture(Types::TypeA, 80.0, &mut mixture, &compatibility).unwrap();
        
        // Add 40 units of TypeB
        // Potential mixture: A=80, B=40. Sum = 120. Saved space = 40 * (1 - 0.25) = 30
        // Potential effective space = 120 - 30 = 90. Still under 100.
        add_to_mixture(Types::TypeB, 40.0, &mut mixture, &compatibility).unwrap();
        assert!((mixture[&Types::TypeA] - 80.0).abs() < f32::EPSILON);
        assert!((mixture[&Types::TypeB] - 40.0).abs() < f32::EPSILON);
        let space = calculate_total_effective_space(&mixture, &compatibility);
        assert!((space - 90.0).abs() < f32::EPSILON);


        // Add another 40 units of TypeB
        // Current: A=80, B=40.
        // Potential after add: A=80, B=80. Sum = 160. Saved space = 80 * (1 - 0.25) = 60
        // Potential effective space = 160 - 60 = 100.
        // This is exactly 100, so no scaling happens.
        add_to_mixture(Types::TypeB, 40.0, &mut mixture, &compatibility).unwrap();
        assert!((mixture[&Types::TypeA] - 80.0).abs() < f32::EPSILON);
        assert!((mixture[&Types::TypeB] - 80.0).abs() < f32::EPSILON);
        let space = calculate_total_effective_space(&mixture, &compatibility);
        assert!((space - 100.0).abs() < f32::EPSILON);


        // Add 10 more units of TypeB
        // Current: A=80, B=80. 
        // Potential: A=80, B=90. Sum = 170. Saved space = 80 * (1 - 0.25) = 60
        // Potential effective space = 170 - 60 = 110. (OVER LIMIT)
        // Scaling factor needed: 100 / 110 = ~0.90909
        // Final amounts should be A = 80 * factor, B = 90 * factor
        add_to_mixture(Types::TypeB, 10.0, &mut mixture, &compatibility).unwrap();
        let factor = 100.0 / 110.0;
        assert!((mixture[&Types::TypeA] - (80.0 * factor)).abs() < f32::EPSILON);
        assert!((mixture[&Types::TypeB] - (90.0 * factor)).abs() < f32::EPSILON);
        let space = calculate_total_effective_space(&mixture, &compatibility);
        assert!((space - 100.0).abs() < f32::EPSILON);
    }
}

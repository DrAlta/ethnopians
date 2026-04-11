use super::FlavorVector;

// 2. BALANCE SCORING (The Dish Quality)
pub fn calculate_balance(fv: &FlavorVector) -> [f32;7] {
    let [
        brightness,
        bitterness,
        saltiness,
        sweetness,
        savoriness,
        richness,
        fieriness,
        ..
 ] = fv.technical;
 
        // 1. Salt reduces bitterness perception
        let final_bitter = (bitterness - (saltiness * 0.3)).max(0.0);

        // 2. Salt enhances brightness (citrus/vinegar pop)
        let final_bright = brightness * (1.0 + saltiness * 0.1);

        // 3. Brightness (Acid) cuts through Richness (Fat)
        let final_rich = (richness - (brightness * 0.2)).max(0.0);

        // 4. Sweetness balances Fieriness (Heat)
        let final_fire = (fieriness - (sweetness * 0.15)).max(0.0);

        // 5. Salt enhances sweetness (SGLT1 multiplier logic)
        let sweet_boost = if saltiness > 0.0 {
            sweetness * (1.0 + 2.5 * saltiness * (-8.0 * saltiness).exp())
        } else {
            sweetness
        };

    [
        final_bright,
        final_bitter,
        saltiness,
        sweet_boost,
        savoriness,
        final_rich,
        final_fire,
    ]
}

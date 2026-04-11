mod calculate_balance;
pub use calculate_balance::calculate_balance;
mod customer;
pub use customer::Customer;
mod score_imbalance;
pub use score_imbalance::score_imbalance;


pub const AROMATICS_SIZE: usize = 2; // [Spicy, Earthy]
pub const TECHNICAL_SIZE: usize = 7; // [brightness, bitterness, saltiness, sweetness, savoriness, richness, fieriness]
#[derive(Clone, Copy, Debug)]
pub struct FlavorVector{
    technical: [f32;TECHNICAL_SIZE],
    aromatics: [f32; AROMATICS_SIZE],
}

pub struct Ingredient {
    name: &'static str,
    potential: FlavorVector,

    technical_fat_solubility: [f32; TECHNICAL_SIZE],   // 0.0 to 1.0
    technical_water_solubility: [f32; TECHNICAL_SIZE], // 0.0 to 1.0

    aromatics_fat_solubility: [f32; AROMATICS_SIZE],   // 0.0 to 1.0
    aromatics_water_solubility: [f32; AROMATICS_SIZE], // 0.0 to 1.0
}



// 1. EXTRACTION LOGIC (The Kitchen)
fn cook_dish(ingredients: &[(Ingredient, f32)], fat_level: f32, water_level: f32) -> FlavorVector {
    let mut technical_final_scores = [0.0; TECHNICAL_SIZE];
    let mut aromatics_final_scores = [0.0; AROMATICS_SIZE];

    for (ing, amount) in ingredients {
        for i in 0..TECHNICAL_SIZE {
            let extraction = (ing.technical_fat_solubility[i] * fat_level) + (ing.technical_water_solubility[i] * water_level);
            // Cap extraction at 100% of potential
            let extracted = (ing.potential.technical[i] * extraction.min(1.0)) * amount;
            technical_final_scores[i] += extracted;
        }


        for i in 0..AROMATICS_SIZE {
            let extraction = (ing.aromatics_fat_solubility[i] * fat_level) + (ing.aromatics_water_solubility[i] * water_level);
            // Cap extraction at 100% of potential
            let extracted = (ing.potential.aromatics[i] * extraction.min(1.0)) * amount;
            aromatics_final_scores[i] += extracted;
        }
    }
    
    FlavorVector{ technical: technical_final_scores, aromatics: aromatics_final_scores }
}



pub fn main() {
    // Example: A customer who wants high heat (+5 relative) and is an "Impossible" Salt Freak (+3 relative)
    let boss_critic = Customer::new(
        "The Fire Marshall",
        10.0,
        45.0, // High intensity expected
        [3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0], 
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8],
        [0.0, 0.0], 
        [0.0, 0.0],
    );

    // Ingredient setup (simplified values)
    let chili = Ingredient {
        name: "Ghost Pepper",
        potential: FlavorVector{technical:[0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 10.0], aromatics: [2.0, 0.0]},
        technical_fat_solubility: [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.9],
        technical_water_solubility: [0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1],
        aromatics_fat_solubility: [0.8, 0.5],
        aromatics_water_solubility: [0.2, 0.1],
    };

    let dish = cook_dish(&[(chili, 1.0)], 0.8, 0.2); // Sautéed
    let score = boss_critic.rate_dish(&dish);

    println!("Critic {} gave the dish a score of: {:.2}", boss_critic.name(), score);
}

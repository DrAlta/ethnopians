/*
oh these are observed trades. it's little compleates are offers because item have Work value, with hoe much time is takes the agasit to produce the item plus the value of the items consumed to produce it.  this it the valued used to accept a trade.  then a has Trade values these are teh market values os what you can trade for.
*/
#[derive(Debug, Clone, Copy)]
struct PriceRange {
    min: f32,
    max: f32,
}

impl PriceRange {
    fn avg(&self) -> f32 { (self.min + self.max) / 2.0 }
    fn scale(&self, factor: f32) -> PriceRange {
        PriceRange { min: self.min * factor, max: self.max * factor }
    }
}

#[derive(Debug, Clone)]
struct ItemProfile {
    name: String,
    time_cost: f32,          // Labor time units
    materials_consumed: f32, // Work value cost of raw input items
    trade_range: PriceRange, // Fluctuating market value band
}

impl ItemProfile {
    // Work Value is a hard mathematical floor: Labor + Ingredients
    fn work_value(&self, labor_rate: f32) -> f32 {
        (self.time_cost * labor_rate) + self.materials_consumed
    }
}


fn observe_and_adjust_market(side_a: PriceRange, side_b: PriceRange) -> (PriceRange, PriceRange) {
    let overlap_min = side_a.min.max(side_b.min);
    let overlap_max = side_a.max.min(side_b.max);

    // If market ranges already overlap, contract them slightly toward the observed mid-point
    if overlap_min <= overlap_max {
        let center = (overlap_min + overlap_max) / 2.0;
        let contract = |r: PriceRange| PriceRange {
            min: r.min + (center - r.min) * 0.1,
            max: r.max - (r.max - center) * 0.1,
        };
        return (contract(side_a), contract(side_b));
    }

    // Disjoint ranges: Apply your inverse quadratic scaling adjustment
    let avg_a = side_a.avg();
    let avg_b = side_b.avg();
    if avg_a <= 0.0 || avg_b <= 0.0 { return (side_a, side_b); }

    let s = (avg_b / avg_a).sqrt();
    (side_a.scale(s), side_b.scale(1.0 / s))
}

fn evaluate_personal_trade(
    agent_labor_rate: f32,
    giving: &ItemProfile,
    receiving: &ItemProfile,
) -> bool {
    // 1. Calculate strict internal work cost floor
    let internal_cost_floor = giving.work_value(agent_labor_rate);
    
    // 2. Look at what the market says the incoming item is worth (average trade value)
    let market_value_gained = receiving.trade_range.avg();

    // 3. Decision rule: Will the agent make a profit over its spent work/materials?
    market_value_gained >= internal_cost_floor
}

fn main() {
    let labor_rate = 5.0; // Agent values its own labor time at 5 coins/hour

    // Setup an item: Iron Sword takes 3 hours to forge and consumes 10 coins of coal/iron
    let mut iron_sword = ItemProfile {
        name: "Iron Sword".to_string(),
        time_cost: 3.0,
        materials_consumed: 10.0, 
        trade_range: PriceRange { min: 15.0, max: 25.0 }, // Avg: 20
    };

    // Setup an observed target item: Exotic Spice
    let mut exotic_spice = ItemProfile {
        name: "Exotic Spice".to_string(),
        time_cost: 0.5,
        materials_consumed: 2.0,
        trade_range: PriceRange { min: 70.0, max: 90.0 }, // Avg: 80
    };

    println!("--- Step 1: Observing a Market Balancing Event ---");
    let (new_a, new_b) = observe_and_adjust_market(iron_sword.trade_range, exotic_spice.trade_range);
    iron_sword.trade_range = new_a;
    exotic_spice.trade_range = new_b;
    
    println!("Adjusted Sword Trade Range Avg: {:.2}", iron_sword.trade_range.avg());
    println!("Adjusted Spice Trade Range Avg: {:.2}", exotic_spice.trade_range.avg());

    println!("\n--- Step 2: Agent Decision Engine ---");
    let sword_work_value = iron_sword.work_value(labor_rate);
    println!("Agent's Work Value floor for Iron Sword: {sword_work_value} coins (3 hrs * 5 rate + 10 mats)");
    println!("Current perceived market value of incoming Spice: {:.2} coins", exotic_spice.trade_range.avg());

    if evaluate_personal_trade(labor_rate, &iron_sword, &exotic_spice) {
        println!("✅ Trade ACCEPTED: The market trade value of Spice covers our labor and material investments.");
    } else {
        println!("❌ Trade REJECTED: Even though market prices shifted, producing a Sword costs more physical work value than the Spice is worth to us.");
    }
}

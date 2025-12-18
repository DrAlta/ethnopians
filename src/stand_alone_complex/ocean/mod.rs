struct OceanProfile {
    traits: [f32; 5], // [O, C, E, A, N]
}
/// zero is nurtal compatability, positive they like them, negative they dislike them
fn compute_directional_like(observer: &OceanProfile, target: &OceanProfile) -> f32 {
    let base_line = compute_frustration(observer, &OceanProfile{traits: [0.5;5]});
    compute_frustration(observer, target) - base_line
}
fn compute_frustration(observer: &OceanProfile, target: &OceanProfile) -> f32 {
    // 1. The Actor Effect: Observer's base level of "satisfiability" 
    // High Neuroticism (N) in the observer lowers their ability to 'like' others.
    let observer_positivity = - (observer.traits[neuroticism] * 0.5);


    // 2. The Partner Effect: Universal traits people find attractive
    // We like people who are kind (A), reliable (C), and stable (low N).
    let target_appeal = (target.traits[agreeableness] * 0.4) + 
                       (target.traits[conscientiousness] * 0.3) + 
                       ((1.0 - target.traits[neuroticism]) * 0.3);

    // 3. Similarity (Openness is the most critical similarity trait)
    let mut total_similarity = 0.0;

    for i in 0..5 {
        let obs_val = observer.traits[i];
        let target_val = target.traits[i];

        // Your specific logic:
        // If observer is high (>0.5), they want the target to be even higher.
        // If observer is low (<0.5), they want the target to be even lower.
        let a = if obs_val >= 0.5 {
            target_val - obs_val
        } else {
            obs_val - target_val
        };

        let a1 = (a.abs() + 1.0).sqrt() - 1.0 ;
        let similarity = if a.is_sign_negative() {-a1} else {a1} + (obs_val - target_val).abs();
        total_similarity += similarity;
    }

// Final directional score: 
    // How much the observer 'likes' the target
    observer_positivity + target_appeal + total_similarity
}

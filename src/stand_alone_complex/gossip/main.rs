fn main() {
    use std::collections::HashMap;

    // Create a HashMap to store all characters in the game.
    let mut all_characters = HashMap::new();

    // --- Create Characters with Personalities and Initial Relationships ---

    // Alice
    let alice = Character {
        id: 1,
        name: String::from("Alice"),
        personality: PersonalityTraits {
            forgiveness_for_common_beliefs: 0.7,
            confirmation_bias: 0.6,
            gullibility_for_confirmation: 0.5,
            conformity: 0.4,
            skepticism: 0.3,
            opinion_weight_bias: 0.0, // Equally weighs affection and trust.
        },
        relationships: HashMap::new(),
        perceptions: HashMap::new(),
    };

    // Bob
    let bob = Character {
        id: 2,
        name: String::from("Bob"),
        personality: PersonalityTraits {
            forgiveness_for_common_beliefs: 0.5,
            confirmation_bias: 0.5,
            gullibility_for_confirmation: 0.5,
            conformity: 0.5,
            skepticism: 0.5,
            opinion_weight_bias: -0.5, // Favors trust over affection.
        },
        relationships: HashMap::new(),
        perceptions: HashMap::new(),
    };

    // Carol
    let carol = Character {
        id: 3,
        name: String::from("Carol"),
        personality: PersonalityTraits {
            forgiveness_for_common_beliefs: 0.6,
            confirmation_bias: 0.7,
            gullibility_for_confirmation: 0.4,
            conformity: 0.5,
            skepticism: 0.6,
            opinion_weight_bias: 0.5, // Favors affection over trust.
        },
        relationships: HashMap::new(),
        perceptions: HashMap::new(),
    };

    // --- Add Characters to the Global Collection ---
    all_characters.insert(alice.id, alice);
    all_characters.insert(bob.id, bob);
    all_characters.insert(carol.id, carol);

    // --- Define Relationships and Perceptions ---

    // Alice's Relationships
    if let Some(alice) = all_characters.get_mut(&1) {
        // Relationship with Bob
        alice.relationships.insert(
            2, // Bob's ID
            Relationship {
                affection: 0.8, // Alice likes Bob a lot.
                trust: 0.9,     // Alice trusts Bob highly.
            },
        );
        // Relationship with Carol
        alice.relationships.insert(
            3, // Carol's ID
            Relationship {
                affection: 0.4, // Alice somewhat likes Carol.
                trust: 0.7,     // Alice somewhat trusts Carol.
            },
        );
    }

    // Bob's Relationships and Perceptions
    if let Some(bob) = all_characters.get_mut(&2) {
        // Relationship with Alice
        bob.relationships.insert(
            1, // Alice's ID
            Relationship {
                affection: 0.6, // Bob likes Alice.
                trust: 0.8,     // Bob trusts Alice.
            },
        );
        // Relationship with Carol
        bob.relationships.insert(
            3, // Carol's ID
            Relationship {
                affection: -0.2, // Bob slightly dislikes Carol.
                trust: 0.5,      // Bob has neutral trust in Carol.
            },
        );

        // Bob's perception of Alice's feelings towards Carol
        bob.perceptions
            .entry(1) // Subject: Alice
            .or_insert_with(HashMap::new)
            .insert(
                3, // Target/Victim: Carol
                Opinion {
                    affection: -0.5, // Bob believes Alice dislikes Carol.
                },
            );
    }

    // Carol's Relationships and Perceptions
    if let Some(carol) = all_characters.get_mut(&3) {
        // Relationship with Alice
        carol.relationships.insert(
            1, // Alice's ID
            Relationship {
                affection: 0.7, // Carol likes Alice.
                trust: 0.6,     // Carol somewhat trusts Alice.
            },
        );
        // Relationship with Bob
        carol.relationships.insert(
            2, // Bob's ID
            Relationship {
                affection: -0.6, // Carol dislikes Bob.
                trust: 0.4,      // Carol doesn't trust Bob much.
            },
        );

        // Carol's perception of Bob's feelings towards Alice
        carol.perceptions
            .entry(2) // Subject: Bob
            .or_insert_with(HashMap::new)
            .insert(
                1, // Target/Victim: Alice
                Opinion {
                    affection: 0.3, // Carol thinks Bob somewhat likes Alice.
                },
            );
    }

    // --- Calculate Consensus Values ---

    // Alice calculates the direct consensus about Carol, excluding Bob.
    if let Some(alice) = all_characters.get(&1) {
        let consensus = alice.direct_consensus(3, Some(2), &all_characters); // Exclude Bob (ID: 2)
        println!("Alice's direct consensus about Carol: {:.2}", consensus);
    }

    // Alice calculates the indirect consensus about how Bob feels about Carol, excluding Bob.
    if let Some(alice) = all_characters.get(&1) {
        let consensus = alice.indirect_consensus(2, 3, Some(2), &all_characters); // Exclude Bob
        println!(
            "Alice's indirect consensus about how Bob feels about Carol: {:.2}",
            consensus
        );
    }
}

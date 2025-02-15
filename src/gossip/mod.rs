//! this was coded by Chad.  I''m reveiw it when I migrate it bevy
use std::collections::HashMap;

/// A unique identifier type for each character in the game.
type CharId = usize;
/// Represents the personality traits of a character that influence their behavior and reactions
/// in the gossip system.
#[derive(Debug, Clone)]
struct PersonalityTraits {
    /// How forgiving the character is when others share commonly held beliefs, even if untrue.
    /// Range: 0.0 (not forgiving) to 1.0 (very forgiving).
    forgiveness_for_common_beliefs: f32,

    /// The extent to which the character prefers information that confirms their own beliefs.
    /// Range: 0.0 (objective/open-minded) to 1.0 (strongly biased towards confirmation).
    confirmation_bias: f32,

    /// How easily the character's trust increases when they hear information that confirms
    /// their existing beliefs.
    /// Range: 0.0 (skeptical) to 1.0 (very gullible).
    gullibility_for_confirmation: f32,

    /// The tendency of the character to align with the opinions of others (conformity).
    /// Range: 0.0 (non-conformist) to 1.0 (highly conformist).
    conformity: f32,

    /// The character's natural skepticism towards new or contradicting information.
    /// Range: 0.0 (not skeptical/gullible) to 1.0 (highly skeptical).
    skepticism: f32,

    /// Determines whether the character weighs affection or trust more when considering others'
    /// opinions. Range: -1.0 (prioritize trust) to 1.0 (prioritize affection).
    opinion_weight_bias: f32,
}
/// Represents the direct relationship between this character and another character.
#[derive(Debug, Clone)]
struct Relationship {
    /// The affection level towards the other character.
    /// Range: -1.0 (strong dislike) to 1.0 (strong liking).
    affection: f32,

    /// The trust level towards the other character.
    /// Range: 0.0 (no trust) to 1.0 (complete trust).
    trust: f32,
}
/// Represents this character's perception of how one character feels about another character.
#[derive(Debug, Clone)]
struct Opinion {
    /// The believed affection that the subject character has towards the target character.
    /// Range: -1.0 (believes the subject strongly dislikes the target) to 1.0 (believes the
    /// subject strongly likes the target).
    affection: f32,
}
/// Represents a character in the game, including their personality, relationships, and perceptions.
#[derive(Debug, Clone)]
struct Character {
    /// Unique identifier for the character.
    id: CharId,

    /// The character's name.
    name: String,

    /// The character's personality traits that influence their gossip behavior and reactions.
    personality: PersonalityTraits,

    /// Direct relationships with other characters.
    /// Key: Other character's ID, Value: Relationship instance.
    relationships: HashMap<CharId, Relationship>,

    /// Perceptions about how other characters feel about one another.
    /// Key: Subject character's ID, Value: Map of target character IDs to Opinions.
    perceptions: HashMap<CharId, HashMap<CharId, Opinion>>,
}
impl Character {
    /// Normalizes a value from the range -1.0 to 1.0 to the range 0.0 to 1.0.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be normalized.
    ///
    /// # Returns
    ///
    /// A normalized value in the range 0.0 to 1.0.
    fn normalize(value: f32) -> f32 {
        (value + 1.0) / 2.0
    }

    /// Calculates the weight assigned to another character's opinion based on this character's
    /// opinion_weight_bias, affection towards the other character, and trust in the other character.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the other character whose opinion weight is being calculated.
    ///
    /// # Returns
    ///
    /// A weight value between 0.0 and 1.0.
    fn calculate_weight(&self, other: &Character) -> f32 {
        // Normalize affection towards the other character to 0.0 - 1.0.
        let affection_weight = Self::normalize(
            self.relationships
                .get(&other.id)
                .map_or(0.0, |rel| rel.affection),
        );

        // Trust weight is taken directly as it's already in the range 0.0 - 1.0.
        let trust_weight = self
            .relationships
            .get(&other.id)
            .map_or(0.0, |rel| rel.trust);

        // Convert opinion_weight_bias from -1.0 - 1.0 to bias factor between 0.0 and 1.0.
        // A value of -1.0 (fully trust-focused) becomes 0.0.
        // A value of 1.0 (fully affection-focused) becomes 1.0.
        let bias = (self.personality.opinion_weight_bias + 1.0) / 2.0;

        // Calculate the final weight by interpolating between affection_weight and trust_weight
        // based on the bias factor.
        bias * affection_weight + (1.0 - bias) * trust_weight
    }
}
impl Character {
    /// Calculates the perceived consensus of direct opinions about the victim among this character's
    /// acquaintances, optionally excluding a specific character.
    ///
    /// # Arguments
    ///
    /// * `victim_id` - The ID of the character about whom the consensus is being calculated.
    /// * `exclude_id` - An optional ID of a character to exclude from the consensus calculation
    ///   (e.g., the gossiper).
    /// * `all_characters` - A reference to a map containing all characters in the game.
    ///
    /// # Returns
    ///
    /// A value between 0.0 and 1.0 representing the consensus opinion, where 0.0 indicates strong
    /// negative consensus and 1.0 indicates strong positive consensus.
    fn direct_consensus(
        &self,
        victim_id: CharId,
        exclude_id: Option<CharId>,
        all_characters: &HashMap<CharId, Character>,
    ) -> f32 {
        let mut sum_weighted_opinions = 0.0; // Accumulates the weighted opinions.
        let mut sum_weights = 0.0;           // Accumulates the weights.

        // Iterate over each acquaintance (characters with whom this character has a relationship).
        for (&other_id, relationship) in &self.relationships {
            // Skip the excluded character, if any.
            if Some(other_id) == exclude_id {
                continue;
            }

            // Retrieve the other character's information.
            if let Some(other) = all_characters.get(&other_id) {
                // Calculate the weight assigned to the other character's opinion.
                let weight = self.calculate_weight(other);

                // Retrieve the other character's affection towards the victim.
                let opinion = Self::normalize(
                    other
                        .relationships
                        .get(&victim_id)
                        .map_or(0.0, |rel| rel.affection),
                );

                // Accumulate the weighted opinion and weight.
                sum_weighted_opinions += weight * opinion;
                sum_weights += weight;
            }
        }

        // Calculate and return the consensus opinion.
        if sum_weights > 0.0 {
            sum_weighted_opinions / sum_weights
        } else {
            0.5 // Neutral consensus if no acquaintances are considered.
        }
    }
}
impl Character {
    /// Calculates the perceived consensus about how the subject feels about the victim, based on
    /// the indirect opinions of this character's acquaintances, optionally excluding a specific character.
    ///
    /// # Arguments
    ///
    /// * `subject_id` - The ID of the subject character whose feelings about the victim are being considered.
    /// * `victim_id` - The ID of the victim character.
    /// * `exclude_id` - An optional ID of a character to exclude from the consensus calculation.
    /// * `all_characters` - A reference to a map containing all characters in the game.
    ///
    /// # Returns
    ///
    /// A value between 0.0 and 1.0 representing the indirect consensus opinion.
    fn indirect_consensus(
        &self,
        subject_id: CharId,
        victim_id: CharId,
        exclude_id: Option<CharId>,
        all_characters: &HashMap<CharId, Character>,
    ) -> f32 {
        let mut sum_weighted_opinions = 0.0; // Accumulates the weighted opinions.
        let mut sum_weights = 0.0;           // Accumulates the weights.

        // Iterate over each acquaintance.
        for (&other_id, _) in &self.relationships {
            // Skip the excluded character, if any.
            if Some(other_id) == exclude_id {
                continue;
            }

            // Retrieve the other character's information.
            if let Some(other) = all_characters.get(&other_id) {
                // Calculate the weight assigned to the other character's opinion.
                let weight = self.calculate_weight(other);

                // Retrieve the other character's opinion about how the subject feels about the victim.
                let opinion = other
                    .perceptions
                    .get(&subject_id)
                    .and_then(|opinions| opinions.get(&victim_id))
                    .map_or(0.0, |op| Self::normalize(op.affection));

                // Accumulate the weighted opinion and weight.
                sum_weighted_opinions += weight * opinion;
                sum_weights += weight;
            }
        }

        // Calculate and return the consensus opinion.
        if sum_weights > 0.0 {
            sum_weighted_opinions / sum_weights
        } else {
            0.5 // Neutral consensus if no acquaintances are considered.
        }
    }
}
/// Represents the potential changes to a character's beliefs resulting from hearing gossip.
struct GossipImpact {
    /// Change in trust towards the gossiper.
    trust_change: f32,

    /// Change in affection towards the victim of the gossip.
    affection_change_towards_victim: f32,

    /// Changes in perceptions about how the subject feels about the victim (for third-party gossip).
    perceptions_update: Option<OpinionChange>,
}

/// Represents a change in perception about how one character feels about another.
struct OpinionChange {
    subject_id: CharId,
    victim_id: CharId,
    affection_change: f32,
}
impl Character {
    /// Calculates the changes to this character's beliefs upon hearing gossip from another character.
    ///
    /// # Arguments
    ///
    /// * `gossiper` - The character who is providing the gossip.
    /// * `gossip_content` - The content of the gossip, encapsulated in a `GossipContent` struct.
    /// * `all_characters` - A reference to a map containing all characters in the game.
    ///
    /// # Returns
    ///
    /// A `GossipImpact` struct containing the calculated changes to trust and opinions.
    fn calculate_gossip_impact(
        &self,
        gossiper: &Character,
        gossip_content: &GossipContent,
        all_characters: &HashMap<CharId, Character>,
    ) -> GossipImpact {
        // Initialize changes
        let mut trust_change = 0.0;
        let mut affection_change_towards_victim = 0.0;
        let mut perceptions_update = None;

        // Determine if the gossip is direct or third-party
        if gossip_content.subject_id == gossiper.id {
            // Direct gossip: Gossiper shares their own opinion about the victim
            let impact = self.calculate_direct_gossip_impact(
                gossiper,
                gossip_content,
                all_characters,
            );

            trust_change = impact.trust_change;
            affection_change_towards_victim = impact.affection_change_towards_victim;
        } else {
            // Third-party gossip: Gossiper shares someone else's opinion about the victim
            let impact = self.calculate_third_party_gossip_impact(
                gossiper,
                gossip_content,
                all_characters,
            );

            trust_change = impact.trust_change;
            perceptions_update = impact.perceptions_update;
        }
        //////////
        // Assuming we've defined a function `get_weighted_social_consensus` as before

        // Get the weighted social consensus about the victim, excluding the gossiper
        let consensus = self.direct_consensus(
            gossip_content.victim_id,
            Some(gossiper.id),
            all_characters,
        );

        // Calculate the forgiveness factor
        let forgiveness = self.personality.forgiveness_for_common_beliefs * consensus;

        // Adjust trust change based on forgiveness
        trust_change *= forgiveness;

        /////////
        // Return the calculated impact
        GossipImpact {
            trust_change,
            affection_change_towards_victim,
            perceptions_update,
        }
    }
}
impl Character {
    /// Calculates the impact of direct gossip on this character's beliefs.
    fn calculate_direct_gossip_impact(
        &self,
        gossiper: &Character,
        gossip_content: &GossipContent,
        all_characters: &HashMap<CharId, Character>,
    ) -> GossipImpact {
        let victim_id = gossip_content.victim_id;

        // 1. Assess alignment between the gossip and this character's existing beliefs about the victim.
        let existing_opinion = self.relationships.get(&victim_id).map_or(0.0, |rel| rel.affection);
        let alignment = 1.0 - (existing_opinion - gossip_content.affection).abs() / 2.0; // Normalize to 0.0 - 1.0

        // 2. Calculate trust change towards the gossiper.
        // Increase trust if gossip aligns with beliefs, decrease if not.
        let gullibility = self.personality.gullibility_for_confirmation;
        let trust_change = if alignment >= 0.5 {
            // Gossip aligns with beliefs
            gullibility * (alignment - 0.5) * 2.0
        } else {
            // Gossip contradicts beliefs
            -self.personality.skepticism * (0.5 - alignment) * 2.0
        };

        // 3. Calculate change in affection towards the victim.
        let confirmation_bias = self.personality.confirmation_bias;
        let affection_change = confirmation_bias * (gossip_content.affection - existing_opinion);

        // Return the impact
        GossipImpact {
            trust_change,
            affection_change_towards_victim: affection_change,
            perceptions_update: None,
        }
    }
}
impl Character {
    /// Calculates the impact of third-party gossip on this character's beliefs.
    fn calculate_third_party_gossip_impact(
        &self,
        gossiper: &Character,
        gossip_content: &GossipContent,
        all_characters: &HashMap<CharId, Character>,
    ) -> GossipImpact {
        let subject_id = gossip_content.subject_id;
        let victim_id = gossip_content.victim_id;

        // 1. Assess credibility of the gossiper.
        let gossiper_trust = self.relationships.get(&gossiper.id).map_or(0.0, |rel| rel.trust);
        let skepticism = self.personality.skepticism;
        let credibility = gossiper_trust * (1.0 - skepticism);

        // 2. Calculate change in perception about how the subject feels about the victim.
        let existing_opinion = self
            .perceptions
            .get(&subject_id)
            .and_then(|opinions| opinions.get(&victim_id))
            .map_or(0.0, |op| op.affection);

        let opinion_change = credibility * (gossip_content.affection - existing_opinion);

        // 3. No direct change in trust towards the gossiper in this example.
        // However, you may implement trust adjustments based on additional factors.

        // Return the impact with perception updates
        GossipImpact {
            trust_change: 0.0,
            affection_change_towards_victim: 0.0, // No direct affection change towards the victim
            perceptions_update: Some(OpinionChange {
                subject_id,
                victim_id,
                affection_change: opinion_change,
            }),
        }
    }
}
/// Represents the content of a gossip shared by a gossiper.
struct GossipContent {
    /// The ID of the subject of the gossip.
    /// If it's direct gossip, `subject_id` is the same as the `gossiper`'s ID.
    subject_id: CharId,

    /// The ID of the victim (the character being talked about).
    victim_id: CharId,

    /// The affection value expressed in the gossip.
    /// Range: -1.0 (strong negative sentiment) to 1.0 (strong positive sentiment).
    affection: f32,
}
impl Character {
    /// Processes gossip received from another character by calculating the impact and updating this
    /// character's beliefs accordingly.
    ///
    /// # Arguments
    ///
    /// * `gossiper` - The character who is providing the gossip.
    /// * `gossip_content` - The content of the gossip.
    /// * `all_characters` - A mutable reference to the map containing all characters in the game.
    fn process_gossip(
        &mut self,
        gossiper: &Character,
        gossip_content: &GossipContent,
        all_characters: &mut HashMap<CharId, Character>,
    ) {
        // Step 1: Calculate the impact of the gossip.
        let impact = self.calculate_gossip_impact(gossiper, gossip_content, all_characters);

        // Step 2: Update trust towards the gossiper.
        self.relationships
            .entry(gossiper.id)
            .and_modify(|rel| {
                rel.trust = (rel.trust + impact.trust_change).clamp(0.0, 1.0);
            })
            .or_insert_with(|| Relationship {
                affection: 0.0,
                trust: impact.trust_change.clamp(0.0, 1.0),
            });

        // Step 3: Update affection towards the victim for direct gossip.
        if gossip_content.subject_id == gossiper.id {
            self.relationships
                .entry(gossip_content.victim_id)
                .and_modify(|rel| {
                    rel.affection = (rel.affection + impact.affection_change_towards_victim)
                        .clamp(-1.0, 1.0);
                })
                .or_insert_with(|| Relationship {
                    affection: impact
                        .affection_change_towards_victim
                        .clamp(-1.0, 1.0),
                    trust: 0.5, // Default trust value for new acquaintances
                });
        }

        // Step 4: Update perceptions for third-party gossip.
        if let Some(opinion_change) = impact.perceptions_update {
            self.perceptions
                .entry(opinion_change.subject_id)
                .or_insert_with(HashMap::new)
                .entry(opinion_change.victim_id)
                .and_modify(|op| {
                    op.affection = (op.affection + opinion_change.affection_change)
                        .clamp(-1.0, 1.0);
                })
                .or_insert(Opinion {
                    affection: opinion_change.affection_change.clamp(-1.0, 1.0),
                });
        }
    }
}

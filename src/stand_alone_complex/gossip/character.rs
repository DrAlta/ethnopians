use std::collections::{BTreeMap, HashMap};

use crate::{Number, stand_alone_complex::gossip::{CharId, GossipContent, GossipImpact, Opinion, OpinionChange, Relationship}};

/// Represents a character in the game, including their personality, relationships, and perceptions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Character {
    /// Unique identifier for the character.
    pub id: CharId,

    /// The character's name.
    pub name: String,

    /// The character's personality traits that influence their gossip behavior and reactions.
    pub personality: super::PersonalityTraits,

    /// Direct relationships with other characters.
    /// Key: Other character's ID, Value: Relationship instance.
    pub relationships: BTreeMap<CharId, super::Relationship>,

    /// Perceptions about how other characters feel about one another.
    /// Key: Subject character's ID, Value: Map of target character IDs to Opinions.
    pub perceptions: BTreeMap<CharId, BTreeMap<CharId, Opinion>>,
}
impl Character {
    /// Normalizes a value from the range -1.0 to 1.0 to the range 0.0 to 1.0.
    ///
    /// This method transforms signed values into a positive normalized scale, 
    /// useful for converting bipolar (-1 to 1) measurements into unipolar (0 to 1) measurements.
    ///
    /// # Key Principles
    /// - Negative values are shifted and scaled down
    /// - Positive values are shifted and scaled up
    /// - Zero remains zero
    /// - Preserves the relative magnitude of the original value
    ///
    /// # Arguments
    /// * `value` - A Number in the range -1.0 to 1.0
    ///
    /// # Returns
    /// A normalized value in the range 0.0 to 1.0
    /// 
    /// # Examples
    /// ```
    /// assert_eq!(Character::normalize(-1.0), 0.0);   // Minimum becomes zero
    /// assert_eq!(Character::normalize(0.0), 0.5);    // Midpoint remains midpoint
    /// assert_eq!(Character::normalize(1.0), 1.0);    // Maximum remains maximum
    /// ```
    fn normalize(value: Number) -> Number {
        (value + Number::ONE) / Number::TWO
    }


    /// Calculates the weight of another character's opinion based on relationship dynamics.
    ///
    /// This method determines how much influence another character's opinion 
    /// should have, considering:
    /// - Affection between characters
    /// - Trust between characters
    /// - The character's bias towards trust or affection
    ///
    /// # Key Principles
    /// - Affection and trust are normalized to 0.0-1.0 range
    /// - A personality bias determines the weight distribution
    /// - Bias of -1.0 means pure trust-based weighting
    /// - Bias of 1.0 means pure affection-based weighting
    ///
    /// # Arguments
    /// * `other` - Reference to another Character whose opinion is being weighted
    ///
    /// # Returns
    /// A weight value between 0.0 and 1.0 representing opinion significance
    fn calculate_weight(&self, other: &Character) -> Number {
        // Normalize affection towards the other character to 0.0 - 1.0.
        let affection_weight: Number = Self::normalize(
            self.relationships
                .get(&other.id)
                .map_or(Number::ZERO, |rel| rel.affection),
        );

        // Trust weight is taken directly as it's already in the range 0.0 - 1.0.
        let trust_weight = self
            .relationships
            .get(&other.id)
            .map_or(Number::ZERO, |rel| rel.trust);

        // Convert opinion_weight_bias from -1.0 - 1.0 to bias factor between 0.0 and 1.0.
        // A value of -1.0 (fully trust-focused) becomes 0.0.
        // A value of 1.0 (fully affection-focused) becomes 1.0.
        let bias = (self.personality.opinion_weight_bias + Number::ONE) / Number::TWO;

        // Calculate the final weight by interpolating between affection_weight and trust_weight
        // based on the bias factor.
        bias * affection_weight + (Number::ONE - bias) * trust_weight
    }
}
impl Character {

    /// Calculates the direct consensus of opinions about a specific character.
    ///
    /// Determines how a group of acquaintances collectively feel about a victim,
    /// excluding an optional character (like the gossiper).
    ///
    /// # Key Principles
    /// - Weights opinions based on relationship strength
    /// - Can exclude specific characters from calculation
    /// - Uses weighted average to determine consensus
    /// - Returns a neutral value if no acquaintances exist
    ///
    /// # Arguments
    /// * `victim_id` - Character ID being evaluated
    /// * `exclude_id` - Optional character ID to exclude from calculation
    /// * `all_characters` - Complete character population
    ///
    /// # Returns
    /// Consensus opinion between 0.0 (very negative) and 1.0 (very positive)
    pub fn direct_consensus(
        &self,
        victim_id: CharId,
        exclude_id: Option<CharId>,
        all_characters: &HashMap<CharId, Character>,
    ) -> Number {
        let mut sum_weighted_opinions = Number::ZERO; // Accumulates the weighted opinions.
        let mut sum_weights = Number::ZERO; // Accumulates the weights.

        // Iterate over each acquaintance (characters with whom this character has a relationship).
        for (&other_id, _relationship) in &self.relationships {
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
                        .map_or(Number::ZERO, |rel| rel.affection),
                );

                // Accumulate the weighted opinion and weight.
                sum_weighted_opinions += weight * opinion;
                sum_weights += weight;
            }
        }

        // Calculate and return the consensus opinion.
        if sum_weights > Number::ZERO {
            sum_weighted_opinions / sum_weights
        } else {
            Number::HALF // Neutral consensus if no acquaintances are considered.
        }
    }
}
impl Character {
    /// Calculates the indirect consensus about how one character feels about another.
    ///
    /// # Key Principles
    /// - Relies on characters' perceptions of others' relationships
    /// - Weights opinions based on relationship credibility
    /// - Can exclude specific characters from calculation
    /// - Provides a nuanced view of social dynamics
    ///
    /// # Arguments
    /// * `subject_id` - Character whose feelings are being assessed
    /// * `victim_id` - Character being evaluated
    /// * `exclude_id` - Optional character to exclude from calculation
    /// * `all_characters` - Complete character population
    ///
    /// # Returns
    /// Indirect consensus opinion between 0.0 (very negative) and 1.0 (very positive)
    /// Represents the perceived feelings of the subject towards the victim
    pub fn indirect_consensus(
        &self,
        subject_id: CharId,
        victim_id: CharId,
        exclude_id: Option<CharId>,
        all_characters: &HashMap<CharId, Character>,
    ) -> Number {
        let mut sum_weighted_opinions = Number::ZERO; // Accumulates the weighted opinions.
        let mut sum_weights = Number::ZERO; // Accumulates the weights.

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
                    .map_or(Number::ZERO, |op| Self::normalize(op.affection));

                // Accumulate the weighted opinion and weight.
                sum_weighted_opinions += weight * opinion;
                sum_weights += weight;
            }
        }

        // Calculate and return the consensus opinion.
        if sum_weights > Number::ZERO {
            sum_weighted_opinions / sum_weights
        } else {
            Number::HALF // Neutral consensus if no acquaintances are considered.
        }
    }
}
impl Character {

    /// Calculates the overall impact of gossip on the character's beliefs and relationships.
    ///
    /// This method is the core of the social dynamics simulation, determining how 
    /// a character processes and internalizes gossip based on:
    /// - Gossip type (direct or third-party)
    /// - Character's personality traits
    /// - Existing relationships and perceptions
    ///
    /// # Key Principles
    /// - Differentiates between direct and third-party gossip
    /// - Applies personality-based filters to gossip reception
    /// - Calculates changes in:
    ///   1. Trust towards the gossiper
    ///   2. Affection towards characters mentioned
    ///   3. Perceptions of relationships
    ///
    /// # Arguments
    /// * `gossiper` - Character providing the gossip
    /// * `gossip_content` - Detailed information about the gossip
    /// * `all_characters` - Complete character population
    ///
    /// # Returns
    /// A `GossipImpact` struct detailing how the gossip changes the character's beliefs
    fn calculate_gossip_impact(
        &self,
        gossiper: &Character,
        gossip_content: &GossipContent,
        all_characters: &HashMap<CharId, Character>,
    ) -> GossipImpact {
        // Initialize changes
        let mut trust_change;
        let mut affection_change_towards_victim = Number::ZERO;
        let mut perceptions_update = None;

        // Determine if the gossip is direct or third-party
        if gossip_content.subject_id == gossiper.id {
            // Direct gossip: Gossiper shares their own opinion about the victim
            let impact =
                self.calculate_direct_gossip_impact(gossiper, gossip_content, all_characters);

            trust_change = impact.trust_change;
            affection_change_towards_victim = impact.affection_change_towards_victim;
        } else {
            // Third-party gossip: Gossiper shares someone else's opinion about the victim
            let impact =
                self.calculate_third_party_gossip_impact(gossiper, gossip_content, all_characters);

            trust_change = impact.trust_change;
            perceptions_update = impact.perceptions_update;
        }
        //////////
        // Assuming we've defined a function `get_weighted_social_consensus` as before

        // Get the weighted social consensus about the victim, excluding the gossiper
        let consensus =
            self.direct_consensus(gossip_content.victim_id, Some(gossiper.id), all_characters);

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

    /// Processes the impact of direct gossip (gossip about the gossiper themselves).
    ///
    /// # Gossip Processing Strategy
    /// 1. Assess alignment between gossip and existing beliefs
    /// 2. Calculate trust change based on belief alignment
    /// 3. Adjust affection towards the victim using confirmation bias
    ///
    /// # Key Principles
    /// - More aligned gossip increases trust
    /// - Contradictory gossip decreases trust
    /// - Confirmation bias influences opinion shifts
    ///
    /// # Arguments
    /// * `gossiper` - Character providing the gossip
    /// * `gossip_content` - Details of the gossip
    /// * `all_characters` - Complete character population
    ///
    /// # Returns
    /// A `GossipImpact` struct detailing changes from direct gossip
    fn calculate_direct_gossip_impact(
        &self,
        _gossiper: &Character,
        gossip_content: &GossipContent,
        _all_characters: &HashMap<CharId, Character>,
    ) -> GossipImpact {
        let victim_id = gossip_content.victim_id;

        // 1. Assess alignment between the gossip and this character's existing beliefs about the victim.
        let existing_opinion = self
            .relationships
            .get(&victim_id)
            .map_or(Number::ZERO, |rel| rel.affection);
        let alignment =
            Number::ONE - (existing_opinion - gossip_content.affection).abs() / Number::TWO; // Normalize to 0.0 - 1.0

        // 2. Calculate trust change towards the gossiper.
        // Increase trust if gossip aligns with beliefs, decrease if not.
        let gullibility = self.personality.gullibility_for_confirmation;
        let trust_change = if alignment >= Number::HALF {
            // Gossip aligns with beliefs
            gullibility * (alignment - Number::HALF) * Number::TWO
        } else {
            // Gossip contradicts beliefs
            -self.personality.skepticism * (Number::HALF - alignment) * Number::TWO
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
    /// Processes the impact of third-party gossip (gossip about another character).
    ///
    /// # Gossip Processing Strategy
    /// 1. Evaluate gossiper's credibility
    /// 2. Calculate perception changes about relationships
    /// 3. Minimal direct trust or affection modifications
    ///
    /// # Key Principles
    /// - Gossiper's trustworthiness affects perception weight
    /// - Focuses on modifying perceptions of relationships
    /// - Skepticism reduces gossip impact
    ///
    /// # Arguments
    /// * `gossiper` - Character providing the gossip
    /// * `gossip_content` - Details of the third-party gossip
    /// * `all_characters` - Complete character population
    ///
    /// # Returns
    /// A `GossipImpact` struct detailing changes from third-party gossip
    fn calculate_third_party_gossip_impact(
        &self,
        gossiper: &Character,
        gossip_content: &GossipContent,
        _all_characters: &HashMap<CharId, Character>,
    ) -> GossipImpact {
        let subject_id = gossip_content.subject_id;
        let victim_id = gossip_content.victim_id;

        // 1. Assess credibility of the gossiper.
        let gossiper_trust = self
            .relationships
            .get(&gossiper.id)
            .map_or(Number::ZERO, |rel| rel.trust);
        let skepticism = self.personality.skepticism;
        let credibility = gossiper_trust * (Number::ONE - skepticism);

        // 2. Calculate change in perception about how the subject feels about the victim.
        let existing_opinion = self
            .perceptions
            .get(&subject_id)
            .and_then(|opinions| opinions.get(&victim_id))
            .map_or(Number::ZERO, |op| op.affection);

        let opinion_change = credibility * (gossip_content.affection - existing_opinion);

        // 3. No direct change in trust towards the gossiper in this example.
        // However, you may implement trust adjustments based on additional factors.

        // Return the impact with perception updates
        GossipImpact {
            trust_change: Number::ZERO,
            affection_change_towards_victim: Number::ZERO, // No direct affection change towards the victim
            perceptions_update: Some(OpinionChange {
                subject_id,
                victim_id,
                affection_change: opinion_change,
            }),
        }
    }
}
impl Character {
    /// Central method for processing and internalizing gossip received from another character.
    ///
    /// # Gossip Processing Workflow
    /// 1. Calculate the overall impact of the gossip
    /// 2. Update trust towards the gossiper
    /// 3. Modify affection towards characters mentioned
    /// 4. Adjust perceptions of relationships
    ///
    /// # Key Principles
    /// - Dynamically updates character's social graph
    /// - Handles both direct and third-party gossip
    /// - Applies personality-driven reaction mechanisms
    /// - Maintains relationship nuance through weighted changes
    ///
    /// # Processing Stages
    /// - Impact Calculation: Determine gossip significance
    /// - Trust Modification: Adjust trust in gossiper
    /// - Affection Update: Change feelings towards mentioned characters
    /// - Perception Refinement: Modify understanding of social relationships
    ///
    /// # Arguments
    /// * `gossiper` - Character providing the gossip
    /// * `gossip_content` - Detailed information about the gossip
    /// * `all_characters` - Mutable map of all characters, allowing global state updates
    ///
    /// # Side Effects
    /// - Modifies the character's internal relationship and perception maps
    /// - Potentially triggers cascading social dynamic changes
    pub fn process_gossip(
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
                rel.trust = (rel.trust + impact.trust_change).clamp(Number::ZERO, Number::ONE);
            })
            .or_insert_with(|| Relationship {
                affection: Number::ZERO,
                trust: impact.trust_change.clamp(Number::ZERO, Number::ONE),
            });

        // Step 3: Update affection towards the victim for direct gossip.
        if gossip_content.subject_id == gossiper.id {
            self.relationships
                .entry(gossip_content.victim_id)
                .and_modify(|rel| {
                    rel.affection = (rel.affection + impact.affection_change_towards_victim)
                        .clamp(Number::NEG_ONE, Number::ONE);
                })
                .or_insert_with(|| Relationship {
                    affection: impact
                        .affection_change_towards_victim
                        .clamp(Number::NEG_ONE, Number::ONE),
                    trust: Number::HALF, // Default trust value for new acquaintances
                });
        }

        // Step 4: Update perceptions for third-party gossip.
        if let Some(opinion_change) = impact.perceptions_update {
            self.perceptions
                .entry(opinion_change.subject_id)
                .or_insert_with(BTreeMap::new)
                .entry(opinion_change.victim_id)
                .and_modify(|op| {
                    op.affection = (op.affection + opinion_change.affection_change)
                        .clamp(Number::NEG_ONE, Number::ONE);
                })
                .or_insert(Opinion {
                    affection: opinion_change
                        .affection_change
                        .clamp(Number::NEG_ONE, Number::ONE),
                });
        }
    }
}

// We’ll use a simple model where each emotional process is represented as a value between 0.0 and 1.0.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct EmotionalState {
    // these are how 'active' the system is 0.0 if inactive. 1.0 is active at full strangth
    /// seeking is increased by novel stimuli, you're walking downa trai land hear one unsual noise you might walk on, but hear a buch and you're curiosity and peeked and to stop to investagate it.
    seeking: f32,
    /// rage is caused by precives  something as taking away your abiliti to adapt. limiting your future state in  'future state maximization' theory themr, to really it's only the future states you where reasoning about and more specificly the ones you where actually considering, if you belaive your would never climb to the top of the bell tower with a sniper riffle and start picking people off, having that future state removed isn't going to stimulate RAGE
    rage: f32,
    /// fear is the response to the detection of threats to our survival and set into motion a biobehavioral response that would facilitate fighting or fleeing
    fear: f32,
    ///
    lust: f32,
    /// care_activation = (perceived_need_the_suject_has_for_protection * kawaii_of_subject) +  (perceived_need_the_suject_has_for_protection * emational_attachment_to_subject)
    care: f32,
    /// panic is caused by the perceived social seperation
    panic: f32,
    play: f32,
}

impl EmotionalState {
    /// Convert the current EmotionalState into a PAD (Pleasure, Arousal, Dominance) tuple.
    /// All output values are normalized to range between -1.0 and 1.0.
    pub fn to_pad(&self) -> (f32, f32, f32) {
        // Pleasure computation:
        //   Positive signals = seeking, lust, care, play.
        //   Negative signals = rage, fear, panic.
        // We calculate the average of positives and negatives, then subtract.
        let pos_avg = (self.seeking + self.lust + self.care + self.play) / 4.0;
        let neg_avg = (self.rage + self.fear + self.panic) / 3.0;
        let mut pleasure = pos_avg - neg_avg;
        // Clamp to [-1, 1] just in case.
        pleasure = pleasure.clamp(-1.0, 1.0);

        // Arousal computation:
        // We use the average level of all emotions as a proxy for arousal.
        // When all values are at 0.5, the result will be neutral.
        let total =
            self.seeking + self.rage + self.fear + self.lust + self.care + self.panic + self.play;
        let arousal_raw = total / 7.0;
        // Map raw arousal from [0, 1] to [-1, 1]:
        let mut arousal = 2.0 * arousal_raw - 1.0;
        arousal = arousal.clamp(-1.0, 1.0);

        // Dominance computation:
        // Empowering signals: rage, lust, seeking.
        // Submissive signals: fear, panic.
        let pos_dom = (self.rage + self.lust + self.seeking) / 3.0;
        let neg_dom = (self.fear + self.panic) / 2.0;
        let mut dominance = pos_dom - neg_dom;
        dominance = dominance.clamp(-1.0, 1.0);

        (pleasure, arousal, dominance)
    }
}

//! # Weapons sizing
//! Regardless of the size of the grip, apply a -2 penalty on a short weapon (1/3 your height or less) used with two hands. It’s just a clumsy thing to do.
//! 
//! # Grip
//! Tiny size and weight don’t make your knife the perfect broadsword for a faerie. The grip is sized all wrong. This is an important part of any weapon design: What size (and how many) hands was it made for? This may place penalties on a Giant trying to use your two-handed sword as an overgrown shortsword.
//! 
//! For simplicity, grip is a “free” part of design, unless you add cost and weight for oddities like a two-handed hilt on a short weapon. 
//! *Note the size of the intended user when designing. 
//! Effects of mismatched grip are very much up to the GM, but -1 skill per level of Size difference is appropriate, as is a -2 or so for two hands on a weapon when there’s only proper room for one. (See GULLIVER for more on gaming these matters.)
//! # Unbalance
//! Unbalance lets a weapon strike harder with a swing; axe users will be glad to see unbalance add some real chop in these rules. But unbalance makes a weapon hard to recover, so use wisely.
//! 
//! Unlike GURPS, weapons can be built here with varying degrees of unbalance. While these are presented as a simple choice of three multipliers, you could of course use any in-between multiplier you choose.
//! # Damage
//! Like GURPS, this system computes damage as base damage plus an addition for the weapon. Base damage comes from ST; the addition, Damage Add, comes from these rules.
//! 
//! Damage Add can be high, leaving results like “1d+6”. Reduce these to damage die: each +3 reduces to 1d damage, each +7 to 2d damage. For example, damage of 1d+9 reduces to 3d+2.
//! 
//! The rules try to achieve GURPS-like damage stats for common weapons used at typical ST levels. Things will be a point or two off at times; a shortsword, for example, gains a bit of thrust damage under these rules, stabbing as effectively as a broadsword. But given that those differences stem from application of consistent rules, you might like the changes they bring!
//! 
//! # Readying time
//! The rules offer Recovery, a statistic indicating how difficult it is to ready a weapon. Think of this as the time required to ready: low Recovery is good, high Recovery is bad. ST makes a big difference in the final effect.
//! 
//! There’s also an attempt below to allow more varied ready times. In GURPS 3e, there are only three very different rates of attack, with nothing in between. The differences among them may be even more extreme than first appears. For example, an axe in GURPS 3e isn’t half as fast as a sword, it’s slower. Over several turns, the sword can perform Attack, Parry / Attack, Parry / Attack, Parry / etc. while the axe is stuck with Attack, Dodge / Ready, Dodge / Attack, Dodge / and so on, if the user wants to be able to attack. He’s getting in half as many attacks as the swordsman, but only one-fourth as many actual uses of the weapon!
//! 
//! Unfortunately, there’s no easy way to game small differences in rate of attack or defense in GURPS. Later below is an optional method of doing so in an abstract way; see if it works for you.
//! # Thrusts
//! Use your thrust damage from the Basic Weapon Damage table (BS 3e p. 74), as always, but based on effective ST, as follows. Strikers and kicks do thrust damage based on full ST; no change there. But punch damage changes. Instead of thrust -2 damage, use simple thrust, based on Combat ST x 2/3 (round down). For example, a ST 10 PC computes punch damage from ST 10 x 2/3 = 6.6, or ST 6. That’s 1d-4.
//! 
//! If you have any item in hand of reasonable mass and hardness (GM call), use Combat ST x 4/5 (round down) instead. This gives you punch damage with a small hard object in hand, a steel gauntlet, brass knuckles etc. Objects with greater heft use the same Combat ST x 4/5 as a base for damage, with Damage Add computed as weapons.
//! 
//! Damage from Combat ST x 4/5 is your base damage for all weapon use. Ignoring very low ST scores, you use 1d-3 for Combat ST 9 to 11, 1d-2 for Combat ST 12 to 13, 1d-1 for Combat ST 14 to 16, 1d for Combat ST 17 to 18, 1d+1 for Combat ST 19 to 21, etc.
//! 
//! To repeat:
//! 
//! *Punch damage is figured from Combat ST x 2/3
//! *Kick and striker damage is figured from Combat ST
//! *Armed damage is figured from Combat ST x 4/5

use crate::Number;



struct Creature {
    // This stricking strangth. it instanances stench of your muscles you can pring into forch when attacking
    combat_st: Number,
    // This is the steanch for listing thing. it is the force of the musles you can use to carry something.
    load_st: Number,
}
impl Creature {
    fn get_combat_st(&self) -> Number {
        self.combat_st.clone()
    }
    fn get_load_st(&self) -> Number {
        self.load_st.clone()
    }
}

fn calc_weapon(
    // the weapons lenth in meters
    length: Number,
    // weapons weight in pounds. here are 453.59237 grams (g) in 1 pound (lb)
    weight: Number,
 ){

}
//! for the test aI just have it plant vegtibles in a field and harvest them ehen they are mature then replant them
//! if out od seed find neared plant to collect seeds from 
//! i thing a veg can be split into 3 seeds
//! useing hands on a plant produces vegs and consumes the plant
//! use an knife of a veg produces 3 seeds and consumes the veg
//! 
//! use a stone on stone produces a knife and consomes one stone
//! 
//! useinga knife on stick or visvera produces a axe and consumes the knife and stick
//! 
//! knife has higher DPS than axe but shorter range
//! 
//! 
//! ----
//! have_2_stone_2
//! ----
//! sel{
//!     invotory have >= 2 stone
//!     seq{
//!         go to stone
//!         take stone
//!     }
//! }
//! ----
//! have_2_stone
//! ----
//! seq {
//!     have_2_stone_2
//!     have_2_stone_2
//! }
//! ---
//!  have_knife
//! ----
//! sel{
//!     inevtor have >= 1 knife
//!     seq {
//!          have_2_stone
//!          combine stone and stone
//!     }
//! 
//! ---
//!  have_stick
//! ---
//! sel{
//!     invitory have >= 1 stick
//!     seq{
//!          go to tree
//!          use hands on tree
//!     }
//!
//! ---
//! have_axe
//! ---
//! sel {
//!  invtory have >= 1 axe
//!  seq {
//!      have_knife
//!      have_stick
//!      combine stick and knife
//!  }
//! ----
//! have_2_wood_2
//! ----
//! sel{
//!     invetory has >= 2 wood
//!     have_axe
//!     go_to_tree
//!     use axe on tree
//! }
//! ----
//! have_2_wood
//! ----
//! seq{
//!     have_2_wood_2
//!     have_2_wood_2
//! }
//! 
//! 
//! ---
//! have house
//! ---
//! 
//! Sel{
//!     is house in range
//!     seq {
//!         have_2_wood
//!         combine wood and wood
//!     }
//! ---
//! sat hunger
//! ---
//! selector{
//!     don't need to eat
//!     seq{
//!         selector{
//!             does he have veg
//!             get_veg
//!         }
//!         eat veg
//!     }
//! }

use std::collections::HashMap;
use super::bt::Instruction::{Action, Selector, Sequence};
pub fn foo() {
    let hermit = "hermit".to_owned();
    // stuff used by hermit
    let sat_hunger = "sat_hunger".to_owned();
    /*
    let sat_sleep = "sat_sleep".to_owned();
    let have_house = "have_house".to_owned();
    let have_garden = "have_garden".to_owned();
    let harvest_veg = "harvest_veg".to_owned();
    let plant_veg = "plant_veg".to_owned();
    // stuff used by sat_hunger
    let dont_need_to_eat = dont_need_to_eat.to_owned();
    // stuff used by 
    // stuff used by 
*/
    let bt = HashMap::from([
        (hermit.clone(), Sequence(vec![
            sat_hunger.clone(),
            sat_sleep.clone(),
            have_house.clone(),
            have_garden.clone(),
            harvest_veg.clone(),
            plant_veg.clone(),
        ])),
        (in)
        (sat_hunger22, Selector(vec![])),
        (sat_hunger2, Sequence(())),
        (sat_hunger, Selector(vec![
            dont_need_to_eat.clone(),
        ])),
    ]);
}
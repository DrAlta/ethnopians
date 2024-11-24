/*
trees: take water and sunlight; produced sticks when used; produce wood when axe in used on them
veggies: take water and sunlight; produce food when used;

food: when used restores energy
wood: used on wood to build house

stone: when used on Stick makes axe
stick: when used on stone makes axe

house: when used restores energy, any excess restores health


---this stuff seems to complex
axe head: make from using stone on stone; used on stick to make axe; used on agent deal damage
grass: take water and sunlight and produce fiber when harvested

Spear: used long range(father than reach of axe) deals damage; use close range pushed target away

*/

pub const MAX_ENERGY: i16 = 100;

pub mod interaction;

mod acts;
pub use acts::Acts;
mod item;
pub use item::Item;
mod location;
pub use location::Location;
mod world;
pub use world::World;

fn within_range(x1:f32, y1:f32, x2:f32, y2:f32, dist: f32) -> bool {
    let x_off= x1 - x2;
    let y_off= y1 - y2;
    ((x_off * x_off) + (y_off * y_off)) > (dist * dist) 
}
/*
impl World {
    pub fn use_item_on_agent(&mut self, agent_idx: usize, direct_idx: usize, other_idx: usize) -> Result<String, String> {
        let Some(agent) = self.agents.get(agent_idx) else {
            return Err("Agent not found!".to_owned());
        };
        let Some(other) = self.agents.get(other_idx) else {
            return Err("Other not found!".to_owned());
        };
        let Some((direct, Location::Agent(direct_agent))) = self.items.get(direct_idx) else {
            return Err("Direct object not found!".to_owned());
        };
        if direct_agent != &agent_idx {
            return Err("agent does not have Direct object!".to_owned());

        }

        /*
        let within_long_range = within_range(agent.x, agent.y, other.x, other.y, 40.0);
        if within_range(agent.x, agent.y, *x, *y, 20.0) || within_long_range && direct == Item::Spear {
        */
        if within_range(agent.x, agent.y, other.x, other.y, 20.0){
            return Err("other is too far away!".to_owned())
        };

        match direct {
            Item::Axe => {
                let other = self.agents.get_mut(other_idx).expect("we have an immutable ref to him so we sohulf be able to get a mut");
                other.hp -= 5;
                if other.hp >= 0 {
                    Ok(format!("{agent_idx} killed {other_idx} with an axe"))
                } else {
                    Ok(format!("{agent_idx} attaked {other_idx} with an axe"))
                }
            },
            Item::Food |
            Item::Stone |
            Item::Stick |
            Item::Wood => Err("you can't use that on someone".to_owned()),
        }
    }
    pub fn use_item_on_building(&mut self, agent_idx: usize, direct_idx: usize, building_idx: usize) -> Result<String, String> {
        let Some(agent) = self.agents.get(agent_idx) else {
            return Err("Agent not found!".to_owned());
        };
        let Some((direct, Location::Agent(direct_agent))) = self.items.get(direct_idx) else {
            return Err("Direct object not found!".to_owned());
        };
        if direct_agent != &agent_idx {
            return Err("agent does not have Direct object!".to_owned());

        }

        let Some((building, x, y)) = self.buildings.get(building_idx) else {
            return Err("building not found!".to_owned());
        };
        if within_range(agent.x, agent.y, *x, *y, 20.0) {
            return Err("building is too far away!".to_owned())
        };

        match (direct, building) {
            (Item::Axe, Building::Tree) => {
                self.buildings.remove(building_idx);
                self.items.push((Item::Wood, Location::Agent(agent_idx)));
                Ok(format!("Wood harvested from {building_idx} by {agent_idx}"))
            },
            (Item::Axe, Building::House) |
            (Item::Axe, Building::Veggie) |
            (Item::Food, Building::House) |
            (Item::Food, Building::Tree) |
            (Item::Food, Building::Veggie) |
            (Item::Stone, Building::House) |
            (Item::Stone, Building::Tree) |
            (Item::Stone, Building::Veggie) |
            (Item::Stick, Building::House) |
            (Item::Stick, Building::Tree) |
            (Item::Stick, Building::Veggie) |
            (Item::Wood, Building::House) |
            (Item::Wood, Building::Tree) |
            (Item::Wood, Building::Veggie) => {
                Err("You can't use that on that".to_owned())
            }
        }
    }

    pub fn use_item_on_item(&mut self, agent_idx: usize, direct_idx: usize, indirect_idx: usize) -> Result<String, String> {
        let Some(agent) = self.agents.get(agent_idx) else {
            return Err("Agent not found!".to_owned());
        };
        let Some((direct, Location::Agent(direct_agent))) = self.items.get(direct_idx) else {
            return Err("Direct object not found!".to_owned());
        };
        if direct_agent != &agent_idx {
            return Err("agent does not have Direct object!".to_owned());

        }

        let Some((indirect, indirect_location)) = self.items.get(indirect_idx) else {
            return Err("Indirect object not found!".to_owned());
        };
        match indirect_location {
            Location::Agent(idx) => {
                if idx != &agent_idx {
                    return Err("someone else has the Indirect object!".to_owned());
                }
            },
            Location::World { x, y } => {
                if within_range(agent.x, agent.y, *x, *y, 20.0) {
                    return Err("indirect object is too far away!".to_owned())
                };
            },
        }

        match (direct, indirect) {
            (Item::Stick, Item::Stone) | (Item::Stone, Item::Stick) => {
                self.items.remove(direct_idx.max(indirect_idx));
                self.items.remove(direct_idx.min(indirect_idx));
                self.items.push((
                    Item::Axe, 
                    Location::Agent(agent_idx)
                ));
                Ok(format!("{agent_idx} created an Axe from {direct_idx} and {indirect_idx}"))
            },
            (Item::Axe, Item::Axe) |
            (Item::Axe, Item::Food) |
            (Item::Axe, Item::Stone) |
            (Item::Axe, Item::Stick) |
            (Item::Axe, Item::Wood) |
            (Item::Food, Item::Axe) |
            (Item::Food, Item::Food) |
            (Item::Food, Item::Stone) |
            (Item::Food, Item::Stick) |
            (Item::Food, Item::Wood) |
            (Item::Stone, Item::Axe) |
            (Item::Stone, Item::Food) |
            (Item::Stone, Item::Stone) |
            (Item::Stone, Item::Wood) |
            (Item::Stick, Item::Axe) |
            (Item::Stick, Item::Food) |
            (Item::Stick, Item::Stick) |
            (Item::Stick, Item::Wood) |
            (Item::Wood, Item::Axe) |
            (Item::Wood, Item::Food) |
            (Item::Wood, Item::Stone) |
            (Item::Wood, Item::Stick) |
            (Item::Wood, Item::Wood) => Err("That can't be used on that.".to_owned()),

        }
    }
}
trait Interaction<O> {
   // const ENUM: E;
    fn available(&self, object: &O) -> bool;
    fn interact(&self, object: &mut O);
}

*/

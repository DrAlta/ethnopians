use std::collections::HashMap;

use super::{Item, Location, World};


#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum Command{
    AddItem{item: Item, loc: Location},
    Remove(usize),
    Damage{agent: usize, ammout: i16},
    Rest{agent_idx: usize, ammount: i16},
    Heal{agent_idx: usize, ammount: i16},
}

#[derive(Debug,PartialEq, PartialOrd, Clone)]
struct Interaction{
//    pub av: fn (&Agent, &Item) ->bool,
    pub act: fn (usize, usize, &World)-> Return,
}

#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum Return {
    ActionInvalid(String),
    Commands(Vec<Command>),
}

fn use_object(agent_idx: usize, direct_object_idx:usize, world: &World) -> Return {
    let Some(Item::Agent) = world.r#type.get(&agent_idx) else {
        return Return::ActionInvalid("Agent not found!".to_owned());
    };

    match world.locations.get(&direct_object_idx) {
        Some(Location::Inventory(inventory)) => {
            if inventory != &agent_idx {
                return Return::ActionInvalid("Object in someone else's inventory".to_owned())
            }
        },
        Some(Location::World { x, y }) => {
            let Some(Location::World{x: agent_x, y: agent_y}) = world.locations.get(&agent_idx) else {
                return Return::ActionInvalid("Actor not in the world with object".to_owned())
            }; 
            if super::within_range(*agent_x, *agent_y, *x, *y, 20.0) {
                return Return::ActionInvalid("object is too far away!".to_owned())
            };
        },
        None => {
            return Return::ActionInvalid("object not found!".to_owned());
        },
    }
    let Some(building) = world.r#type.get(&direct_object_idx) else {
        return Return::ActionInvalid("object not found!".to_owned());
    };
    match building {
        Item::Agent => todo!(),
        Item::Axe => todo!(),
        Item::Food => todo!(),
        Item::Stone => todo!(),
        Item::Stick => todo!(),
        Item::Wood => todo!(),
        Item::House => {
            let Some(hp) = world.hp.get(&agent_idx) else {
                return Return::ActionInvalid("agent doesn't have hp".to_owned())
            };
            let excess = ((hp + 10) - 100).max(0);
            let rest= 10 - excess;
            let mut ret = vec![Command::Rest{agent_idx, ammount: rest}];

            if excess != 0 {
                ret.push(Command::Heal{agent_idx, ammount: excess})
            }
            return Return::Commands(ret)
        },
        Item::Tree => { 
            let Some(direct_idx) = world.r#type.iter().find_map(|(idx, obj)|{
                if obj == &Item::Axe && Some(&Location::Inventory(agent_idx)) == world.locations.get(idx) { 
                    Some(idx)
                } else {
                    None
                }
            }) else {
                return Return::ActionInvalid("Agent doesn't have an axe!".to_owned());
            };
            return Return::Commands(vec![
                Command::Remove(direct_object_idx),
                Command::AddItem{item:Item::Wood, loc:Location::Inventory(agent_idx)}
            ])
        },
        Item::Veggie => {
            return Return::Commands(vec![
                Command::Remove(direct_object_idx),
                Command::AddItem{item:Item::Food, loc:Location::Inventory(agent_idx)}
            ])
        },
    }
}

fn get_interactions() -> Vec<Interaction> {
    vec![
        Interaction{ act: use_object }
    ]
}

fn foofoo(ag: usize, direct_object:usize, world: &World) -> Vec<usize>{
    get_interactions().iter().enumerate().filter_map(|(idx, act)| {
        let c = (act.act)(ag, direct_object, world);

        if let Return::ActionInvalid(err) = c  {
            println!("testing action {idx} got {err}");
            None
        } else {
            Some(idx)
        }
    }).collect()
}

pub fn main(){
    let world = super::World{ 
        locations: HashMap::from([
            (0, Location::World { x: 0.0, y: 0.0 }),
            (1, Location::Inventory(0)),
            (2, Location::World { x: 10.0, y: 0.0 }),
        ]),
        hp: HashMap::from([
            (0, 10)
        ]),
        r#type: HashMap::from([
            (0, Item::Agent),
            (1, Item::Axe),
            (2, Item::Tree),
        ]),
    };

    let available_commands = foofoo(
        0, 
        2,
        &world
    );

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    pub fn over_test() {
        let world = super::World{ 
            locations: HashMap::from([
                (0, Location::World { x: 0.0, y: 0.0 }),
                (1, Location::Inventory(0)),
                (2, Location::World { x: 10.0, y: 0.0 }),
            ]),
            hp: HashMap::from([
                (0, 10)
            ]),
            r#type: HashMap::from([
                (0, Item::Agent),
                (1, Item::Axe),
                (2, Item::Tree),
            ]),
        };
    
        let available_commands = foofoo(
            0, 
            2,
            &world
        );
    


        assert_eq!(available_commands, vec![0]);
    }
}
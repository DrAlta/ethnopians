use std::collections::HashMap;

mod r#use;

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


fn get_interactions() -> Vec<Interaction> {
    vec![
        Interaction{ act: r#use::use_object }
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
        energy: HashMap::from([
            (0, 10)
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
    let acts = get_interactions();
    println!("Available actions:");
    for idx in &available_commands {
        println!("{idx}:{:?}", acts[*idx]);
    }
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
            energy: HashMap::from([
                (0, 10)
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
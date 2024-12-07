use super::{Return, UseObject, World};



#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub struct Interaction<Command>{
//    pub av: fn (&Agent, &Item) ->bool,
    pub name: String,
    pub act: fn (usize, usize, &World)-> Return<Command>,
}

impl<Command> std::fmt::Display for Interaction<Command> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}


pub fn get_interactions<Command: UseObject<Command>>() -> Vec<Interaction<Command>> {
    vec![
        Interaction{ 
            name: "use".into(),
            act: Command::use_object 
        }
    ]
}

pub fn foofoo<Command: UseObject<Command>>(ag: usize, direct_object:usize, world: &World) -> Vec<usize>{
    get_interactions().iter().enumerate().filter_map(|(idx, act)| {
        let c: Return<Command> = (act.act)(ag, direct_object, world);

        if let Return::ActionInvalid(err) = c  {
            println!("testing action {idx} got {err}");
            None
        } else {
            Some(idx)
        }
    }).collect()
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::sandbox::{Command, Item, Location};

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
    
        let available_commands = foofoo::<Command>(
            0, 
            2,
            &world
        );
    


        assert_eq!(available_commands, vec![0]);
    }
}
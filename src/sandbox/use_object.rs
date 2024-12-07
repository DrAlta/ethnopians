use crate::sandbox::World;

use super::Return;

pub trait UseObject<Command> {
    fn use_object(agent_idx: usize, object_idx:usize, world: &World) -> Return<Command>;

}
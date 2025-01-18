use crate::sandbox::World;

use super::{EntityId, Return};

pub trait UseObject<Command> {
    fn use_object(agent_idx: EntityId, object_idx: EntityId, world: &World) -> Return<Command>;
}

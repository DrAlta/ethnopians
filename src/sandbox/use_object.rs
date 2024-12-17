use crate::sandbox::World;

use super::{ObjectId, Return};

pub trait UseObject<Command> {
    fn use_object(agent_idx: ObjectId, object_idx: ObjectId, world: &World) -> Return<Command>;
}

use crate::sandbox::EntityId;

#[allow(dead_code)]
pub enum Avalibility {
    From(EntityId),
    Collision(EntityId),
    RearEnded(EntityId),
}

use crate::sandbox::ObjectId;

#[allow(dead_code)]
pub enum Avalibility {
    From(ObjectId),
    Collision(ObjectId),
    RearEnded(ObjectId),
}

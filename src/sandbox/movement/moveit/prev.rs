use crate::sandbox::EntityId;

pub trait Prev {
    fn get_location(&self, id: &EntityId) -> Option<(f32, f32)>;
    fn get_size(&self, id: &EntityId) -> Option<(f32, f32)>;
}

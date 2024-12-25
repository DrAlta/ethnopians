use crate::sandbox::ObjectId;

pub trait Prev {
    fn get_location(&self, id: &ObjectId) -> Option<(f32, f32)>;
    fn get_size(&self, id: &ObjectId) -> Option<(f32, f32)>;
}
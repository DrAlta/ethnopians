use crate::{sandbox::{
    world::{Movement, Size},
    EntityId, Location,
}, Number};
use bevy::prelude::*;

pub trait Prev {
    fn get_location(&self, id: EntityId) -> Option<(Number, Number)>;
    fn get_size(&self, id: EntityId) -> Option<(Number, Number)>;
}

impl<'a, 'b, 'c, 'd, 'e> Prev
    for Query<
        'd,
        'e,
        (
            EntityId,
            Option<&'a Movement>,
            Option<&'b mut Location>,
            &'c Size,
        ),
    >
{
    fn get_location(&self, id: EntityId) -> Option<(Number, Number)> {
        let (_, _, location_maybe, _) = self.get(id).ok()?;
        let Some(Location::World { x, y }) = location_maybe else {
            return None;
        };
        Some((*x, *y))
    }

    fn get_size(&self, id: EntityId) -> Option<(Number, Number)> {
        let (_, _, _, Size { width, height }) = self.get(id).ok()?;
        Some((*width as Number, *height as Number))
    }
}

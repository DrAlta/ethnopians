use crate::{
    sandbox::EntityId,
    Number,
};

#[cfg(feature = "bevy")]
use crate::
    sandbox::{
        world::{Movement, Size},
        Location,
    };

pub trait Prev {
    fn get_location(&self, id: EntityId) -> Option<(Number, Number)>;
    fn get_size(&self, id: EntityId) -> Option<(Number, Number)>;
}

#[cfg(feature = "bevy")]
impl<'a, 'b, 'c, 'd, 'e> Prev
    for bevy::prelude::Query<
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
        Some((Into::<Number>::into(*width), Into::<Number>::into(*height)))
    }
}

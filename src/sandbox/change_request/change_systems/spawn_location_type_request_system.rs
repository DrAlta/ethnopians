use bevy::prelude::*;
use qol::logy;

use crate::sandbox::{change_request::ChangeSpawnLocationType, world::Type};

pub fn spawn_location_type_request_system(
    mut requests: EventReader<ChangeSpawnLocationType>,
    mut commands: Commands,
) {
    for ChangeSpawnLocationType { location, tyep } in requests.read() {
        logy!(
            "trace-req-spawn-location-type",
            "spawning entity of type {tyep:?}with location: {location:?}"
        );
        commands.spawn((Type(tyep.clone()), location.clone()));
    }
}

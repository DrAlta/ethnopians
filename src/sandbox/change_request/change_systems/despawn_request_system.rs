use bevy::prelude::*;
use qol::logy;

use crate::sandbox::change_request::ChangeDespawn;

pub fn despawn_request_system(
    mut requests: EventReader<ChangeDespawn>,
    mut commands: Commands,
) {
    for ChangeDespawn(entity_id) in requests.read() {
        logy!("trace-req-despawn", "despawning entity {entity_id:?}");
        commands.entity(*entity_id).despawn();
    }

}
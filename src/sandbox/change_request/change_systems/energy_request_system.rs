use bevy::prelude::*;
use qol::logy;

use crate::sandbox::{change_request::ChangeEnergy, world::Energy};

pub fn energy_request_system(
    mut query: Query<&mut Energy>,
    mut requests: EventReader<ChangeEnergy>,
) {
    for ChangeEnergy { entity_id, delta } in requests.read() {

        let Ok(mut energy) = query.get_mut(*entity_id) else {
            logy!("error", "there was no entity {entity_id:?} with an Energy to change");
            continue;
        };
        logy!("trace-req-energy", "modifing Energy on entity {entity_id:?}");
        energy.0 += delta;
    }

}
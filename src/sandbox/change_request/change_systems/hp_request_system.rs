use bevy::prelude::*;
use qol::logy;

use crate::sandbox::{change_request::ChangeHp, world::Hp};

pub fn hp_request_system(
    mut query: Query<&mut Hp>,
    mut requests: EventReader<ChangeHp>,
) {
    for ChangeHp { entity_id, delta } in requests.read() {

        let Ok(mut hp) = query.get_mut(*entity_id) else {
            logy!("error", "there was no entity {entity_id:?} with a Hp to change");
            continue;
        };
        logy!("trace-req-hp", "modifing Hp on entity {entity_id:?}");
        hp.0 += delta;
    }

}
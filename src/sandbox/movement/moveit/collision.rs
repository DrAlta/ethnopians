use std::collections::{BTreeSet, HashMap};

use broad_phase::{AARect, Entity, EntityId, SpatialBloom};

use crate::sandbox::{ObjectId, Prev};

use super::Avalibility;

pub fn collision<T: Prev>(
    mut todo: BTreeSet<ObjectId>,
    aval: &mut HashMap<EntityId, Avalibility>,
    map: &mut SpatialBloom,
    prev: &T,
) {
    loop {
        let Some(unit_id) = todo.pop_first() else {
            return;
        };
        let Some((x, y)) = prev.get_location(&unit_id) else {
            continue;
        };
        let Some(size) = prev.get_size(&unit_id) else {
            continue;
        };
        let q = map.qurry(x.clone(), y.clone(), x + size.0, y + size.1);
        let mut add_rearended = false;
        for k in q {
            match aval.get_mut(&k) {
                Some(cell @ Avalibility::From(_)) => {
                    let Avalibility::From(o) = cell else {
                        continue;
                    };
                    if o == &unit_id {
                        add_rearended = true;
                    }
                    let o2 = o.clone();
                    if let Avalibility::From(source_object) =
                        std::mem::replace(cell, Avalibility::Collision(o2))
                    {
                        todo.insert(source_object);
                    };
                    continue;
                }
                Some(Avalibility::Collision(_)) => (),
                Some(Avalibility::RearEnded(_)) => (),
                None => (),
            }
        }
        if add_rearended {
            let k = map.insert(Entity::AARect(AARect::new(
                x.clone(),
                y.clone(),
                size.0,
                size.1,
            )));
            aval.insert(k, Avalibility::RearEnded(unit_id.clone()));
        }
    }
}

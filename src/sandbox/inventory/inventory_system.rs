use std::collections::{BTreeSet, HashMap};

use bevy::prelude::*;

use qol::InsertOrInsert;

use crate::sandbox::{inventory::Inventory, EntityId, Location};

/// inventory_system makes sure the Inventory components are up to day with
/// the items that have location of being in their container's inventory
pub fn inventory_system(
    location_query: Query<(Entity, &Location)>,
    mut inventory_query: Query<(Entity, &mut Inventory)>,
    mut commands: Commands,
) {
    let mut todo = HashMap::<EntityId, BTreeSet<EntityId>>::new();

    // build of a list of all the items in each inventory
    for (object_id, location) in location_query.iter() {
        if let &Location::Inventory(container) = location {
            todo.insert_or_insert(container, object_id);
        }
    }
    // replated the esisting invetories with the new items or clear it is there
    // is nothing in it now.
    for (container_id, mut inventory) in inventory_query.iter_mut() {
        let Some(items) = todo.remove(&container_id) else {
            inventory.0.clear();
            continue;
        };
        inventory.0 = items;
    }
    // add new invitories for what ever is left.
    for (entity_id, items) in todo {
        commands.entity(entity_id).insert(Inventory(items));
    }
}

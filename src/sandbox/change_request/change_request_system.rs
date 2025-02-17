use std::collections::{BTreeMap, BTreeSet, HashMap};

use bevy::prelude::*;

use qol::PushOrInsert;

use crate::sandbox::change_request::{ChangeConflict, ChangeRequest, Changes, Dispatch};

type ChangeCollision = ChangeConflict;
type Hash = u64;

pub fn change_request_system(
    mut requests: EventReader<ChangeRequest>,
    mut conflicts: EventWriter<ChangeConflict>,
    mut commands: Commands,
) {
    let mut change_requests_by_contentous_entities = HashMap::<Entity, Vec<Hash>>::new();
    let mut change_requests = BTreeMap::<Hash, (&BTreeSet<Entity>, &Vec<Changes>)>::new();
    let mut collisions = Vec::new();

    for ChangeRequest {
        hash,
        contentious_entities,
        changes,
    } in requests.read()
    {
        if change_requests.contains_key(hash) {
            collisions.push(hash.clone());
            conflicts.send(ChangeCollision { hash: hash.clone() });
        } else {
            change_requests.insert(hash.clone(), (contentious_entities, changes));
            for &contentious in contentious_entities {
                change_requests_by_contentous_entities.push_or_insert(contentious, hash.clone());
            }
        }
    }

    // remove hash collision
    for (_, vec) in change_requests_by_contentous_entities.iter_mut() {
        vec.retain(|x| !collisions.contains(x));
    }

    for (request_hash, &(contentious_entities, changes)) in &change_requests {
        if collisions.contains(request_hash) {
            continue;
        }
        let mut cleared = true;
        for contentious in contentious_entities {
            if let Some(thing) = change_requests_by_contentous_entities.get_mut(contentious) {
                thing.retain(|x| x != request_hash);
                if !thing.is_empty() {
                    cleared = false;
                }
            }
        }
        if cleared {
            changes.dispatch(&mut commands);
        } else {
            conflicts.send(ChangeConflict {
                hash: request_hash.clone(),
            });
        }
    }
    /*
     go down the sorted by hash list and if the request if righting remove it and update the
         let mut cleared = true;
        for x in request.contentous {
             if let Some(thing) = change_requests_by_contentous_entities.get_mut(x) {
                 thing.remove(request)
                 if !thing.is_Empty() {
                     cleared = false;
                 }
             }
        }
         if cleared {
             send_event_for_Change_to_happen
        }
    */
}

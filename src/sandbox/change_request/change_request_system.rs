use std::collections::{BTreeMap, BTreeSet, HashMap};

use bevy::prelude::*;

use qol::PushOrInsert;

use crate::sandbox::change_request::{Changes, ChangeRequest, Dispatch};

type Hash = u64;

pub fn change_request_system(
    mut requests: EventReader<ChangeRequest>,
    mut commands: Commands,
){
    let mut change_requests_by_contentous_entities =  HashMap::<Entity, Vec<Hash>>::new();
    let mut change_requests = BTreeMap::<Hash, (&BTreeSet<Entity>, &Vec<Changes>)>::new();

    for ChangeRequest { hash, contentious_entities, changes } in requests.read(){
        for &contentious in contentious_entities {
            change_requests.insert(hash.clone(), (contentious_entities, changes));
            change_requests_by_contentous_entities.push_or_insert(contentious, hash.clone());
        }
    }

    for (request_hash, &(contentious_entities, changes)) in &change_requests {
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
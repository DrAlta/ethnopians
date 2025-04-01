use bevy::prelude::*;

use crate::{
    sandbox::{
        actions::use_object::UseRequest,
        posible_actions::{PosibleActionsRequest, PosibleActionsResponce},
        world::Type,
        Item, Location,
    },
    types::ActionId,
    Number,
};

use super::*;
/*
#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct AgentIdObjectId(EntityId, EntityId);
fn test_system(
    query: Query<(Entity, &AgentIdObjectId)>,
    mut events: EventWriter<PosibleActionsRequest>,
    mut commands: Commands,
) {
    for (id, &AgentIdObjectId(agent_id, target_id)) in &query {
        events.send(PosibleActionsRequest {
            agent_id,
            target_id,
        });
        commands.entity(id).despawn();
    }
}
*/
#[test]
pub fn no_agent_test() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    app.add_event::<UseRequest>();
    app.add_event::<PosibleActionsRequest>();
    app.add_event::<PosibleActionsResponce>();
    app.add_systems(Update, use_object_system);
    let target_id = app.world_mut().spawn(Type(Item::Food)).id();

    app.world_mut().send_event(PosibleActionsRequest {
        agent_id: Entity::from_raw(0),
        target_id,
    });

    app.update();

    let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
    let mut response_reader = response_events.get_cursor();
    assert!(response_reader.read(response_events).next().is_none());
}

#[test]
pub fn someones_else_object_test() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    app.add_event::<UseRequest>();
    app.add_event::<PosibleActionsRequest>();
    app.add_event::<PosibleActionsResponce>();
    app.add_systems(Update, use_object_system);
    let owner_id = app.world_mut().spawn(Type(Item::Food)).id();
    let target_id = app
        .world_mut()
        .spawn((Type(Item::Food), Location::Inventory(owner_id)))
        .id();
    let agent_id = app.world_mut().spawn(Type(Item::Agent)).id();

    app.world_mut().send_event(PosibleActionsRequest {
        agent_id,
        target_id,
    });

    app.update();

    let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
    let mut response_reader = response_events.get_cursor();
    assert!(response_reader.read(response_events).next().is_none());
}
#[test]
pub fn agent_in_another_world_test() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    app.add_event::<UseRequest>();
    app.add_event::<PosibleActionsRequest>();
    app.add_event::<PosibleActionsResponce>();
    app.add_systems(Update, use_object_system);
    let world_id = app.world_mut().spawn(Type(Item::House)).id();
    let agent_id = app
        .world_mut()
        .spawn((Type(Item::Agent), Location::Inventory(world_id)))
        .id();
    let target_id = app
        .world_mut()
        .spawn((
            Type(Item::Veggie),
            Location::World {
                x: Number::ONE,
                y: Number::ONE,
            },
        ))
        .id();

    app.world_mut().send_event(PosibleActionsRequest {
        agent_id,
        target_id,
    });

    app.update();

    let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
    let mut response_reader = response_events.get_cursor();
    assert!(response_reader.read(response_events).next().is_none());
}
#[test]
pub fn too_far_test() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    app.add_event::<UseRequest>();
    app.add_event::<PosibleActionsRequest>();
    app.add_event::<PosibleActionsResponce>();
    app.add_systems(Update, use_object_system);
    let target_id = app
        .world_mut()
        .spawn((
            Type(Item::Veggie),
            Location::World {
                x: Number::ZERO,
                y: Number::ZERO,
            },
        ))
        .id();
    let agent_id = app
        .world_mut()
        .spawn((
            Type(Item::Agent),
            Location::World {
                x: Number::ZERO,
                y: Into::<Number>::into(100.0),
            },
        ))
        .id();

    app.world_mut().send_event(PosibleActionsRequest {
        agent_id,
        target_id,
    });
    app.update();

    let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
    let mut response_reader = response_events.get_cursor();
    assert!(response_reader.read(response_events).next().is_none());
}
#[test]
pub fn no_object_location_test() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    app.add_event::<UseRequest>();
    app.add_event::<PosibleActionsRequest>();
    app.add_event::<PosibleActionsResponce>();
    app.add_systems(Update, use_object_system);
    let target_id = app.world_mut().spawn(Type(Item::Veggie)).id();
    let agent_id = app
        .world_mut()
        .spawn((
            Type(Item::Agent),
            Location::World {
                x: Number::ZERO,
                y: Number::ZERO,
            },
        ))
        .id();

    app.world_mut().send_event(PosibleActionsRequest {
        agent_id,
        target_id,
    });

    app.update();

    let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
    let mut response_reader = response_events.get_cursor();
    assert!(response_reader.read(response_events).next().is_none());
}
#[test]
pub fn no_object_type_test() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    app.add_event::<UseRequest>();
    app.add_event::<PosibleActionsRequest>();
    app.add_event::<PosibleActionsResponce>();
    app.add_systems(Update, use_object_system);
    let target_id = app
        .world_mut()
        .spawn(Location::World {
            x: Number::ZERO,
            y: Number::ZERO,
        })
        .id();
    let agent_id = app
        .world_mut()
        .spawn((
            Type(Item::Agent),
            Location::World {
                x: Number::ZERO,
                y: Number::ZERO,
            },
        ))
        .id();

    app.world_mut().send_event(PosibleActionsRequest {
        agent_id,
        target_id,
    });

    app.update();

    let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
    let mut response_reader = response_events.get_cursor();
    assert!(response_reader.read(response_events).next().is_none());
}
#[test]
pub fn use_test() {
    //std::env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    app.add_event::<UseRequest>();
    app.add_event::<PosibleActionsRequest>();
    app.add_event::<PosibleActionsResponce>();
    app.add_systems(Update, use_object_system);

    let agent_id = app
        .world_mut()
        .spawn((
            Type(Item::Agent),
            Location::World {
                x: Number::ZERO,
                y: Number::ZERO,
            },
        ))
        .id();
    let target_id = app
        .world_mut()
        .spawn((Type(Item::Veggie), Location::Inventory(agent_id)))
        .id();

    app.world_mut().send_event(PosibleActionsRequest {
        agent_id,
        target_id,
    });

    app.update();

    let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
    let mut response_reader = response_events.get_cursor();
    assert_eq!(
        response_reader.read(response_events).next(),
        Some(&PosibleActionsResponce {
            agent_id,
            target_id,
            action_id: ActionId::USE_OBJECT
        })
    );
}

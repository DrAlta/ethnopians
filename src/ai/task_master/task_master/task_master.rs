use crate::ai::task_master::task_master::TickCount;
use crate::ai::task_master::SubSystemState;
use crate::ai::{Blackboard, BlackboardKey, BlackboardValue, Prayer};

#[derive(Debug, Clone, PartialEq)]
pub struct TaskMaster<InpulseId, EntityId/*: std::hash::Hash + std::fmt::Debug*/, Item> {
    pub(in crate::ai::task_master) blackboard: Blackboard<BlackboardKey, BlackboardValue<EntityId>>,
    pub(in crate::ai::task_master) stack: Vec<SubSystemState<EntityId>>,
    pub(in crate::ai::task_master) tick_counter: TickCount,
    pub(in crate::ai::task_master) prayer_being_waited_on_maybe:
        Option<(Prayer<InpulseId, EntityId, Item>, TickCount)>,
}

impl<InpulseId, EntityId, Item> TaskMaster<InpulseId, EntityId, Item> {
    pub fn new(
        blackboard: Blackboard<BlackboardKey, BlackboardValue<EntityId>>,
        stack: Vec<SubSystemState<EntityId>>,
        tick_counter: TickCount,
        prayer_being_waited_on_maybe:
            Option<(Prayer<InpulseId, EntityId, Item>, TickCount)>,        
    )-> Self{
        TaskMaster { blackboard, stack, tick_counter, prayer_being_waited_on_maybe }
    }
}
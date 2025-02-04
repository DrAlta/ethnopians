use crate::sandbox::EntityId;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum StackItem {
    //Behaior states
    Sequence(usize),
    Selector(usize),
    // return statues
    Success,
    Failure,
    Init,
    Int(i32),
    True,
    False,
    Coord { x: i32, y: i32 },
    String(String),
    EntityId(EntityId),
    Option(Option<Box<StackItem>>),
    Todo(Vec<EntityId>),
}
impl StackItem {
    pub fn some(value: StackItem) -> StackItem {
        Self::Option(Some(Box::new(value)))
    }
    pub fn none() -> StackItem {
        Self::Option(None)
    }
}

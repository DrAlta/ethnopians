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
}

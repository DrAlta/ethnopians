#[derive(Debug, PartialEq)]
pub enum StackItem {
    //node states
    Sequence(usize),
    Selector(usize),
    // return statues
    Success,
    Failure,
    Init,
}

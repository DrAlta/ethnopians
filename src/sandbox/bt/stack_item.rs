#[derive(Debug, PartialEq)]
pub enum StackItem {
    //Behaior states
    Sequence(usize),
    Selector(usize),
    // return statues
    Success,
    Failure,
    Init,
}

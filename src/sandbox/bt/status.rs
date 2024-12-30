use super::ActionId;

#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
    Failure,
    Running(ActionId),
    None,
}

use super::ActionId;

#[derive(Debug)]
pub enum Status {
    Success,
    Failure,
    Running(ActionId),
    None,
}

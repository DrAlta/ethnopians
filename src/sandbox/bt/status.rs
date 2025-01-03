use super::InpulseId;

#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
    Failure,
    Running(InpulseId),
    None,
}

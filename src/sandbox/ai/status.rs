use crate::sandbox::ai::InpulseId;

#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
    Failure,
    Running(InpulseId),
    None,
}

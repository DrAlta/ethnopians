use crate::sandbox::new_ai::Prayer;

#[derive(Debug, Clone, PartialEq)]
pub enum TastMasterReport {
    Prayer(Prayer),
    WaitingOnPrayer,
    Err(String),
    Ok,
    Success,
    Failure{reason: String},
}

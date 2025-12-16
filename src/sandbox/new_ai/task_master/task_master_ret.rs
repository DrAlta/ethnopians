use crate::sandbox::new_ai::Prayer;

#[derive(Debug, Clone, PartialEq)]
pub enum TastMasterRet {
    Prayer(Prayer),
    WaitingOnPrayer,
    Err(String),
    Ok,
}

use crate::ai::Prayer;

#[derive(Debug, Clone, PartialEq)]
pub enum TastMasterReport<InpulseId, EntityId, Item> {
    Prayer(Prayer<InpulseId, EntityId, Item>),
    WaitingOnPrayer,
    Err(String),
    Ok,
    Success,
    Failure { reason: String },
}

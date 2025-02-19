mod deep_copy;
mod stack_item;
pub use stack_item::StackItem;
mod table;
pub(in crate::sandbox) use table::TableInterior;

//! Seq{
//!     a,
//!     b,
//!     c,
//! }
//! (seq{_2,_3,_4}, {_2:a, _3:b, _4:c})
//!

use crate::sandbox::ai::{parser::TreesUsed, Thread};

/*
mod action_parser;
pub use action_parser::action_parser;
*/
mod blackboard_parser;
pub use blackboard_parser::blackboard_parser;
mod combine_parser;
pub use combine_parser::combine_parser;
mod eat_parser;
pub use eat_parser::eat_parser;
mod file_parser;
pub use file_parser::{file_parser, named_tree_parser};
mod forth_tree_parser;
pub use forth_tree_parser::forth_tree_parser;
mod inventory_have_ge_parser;
pub use inventory_have_ge_parser::inventory_have_ge_parser;
mod selector;
pub use selector::selector_parser;
mod sequence;
pub use sequence::sequence_parser;
mod token_parser;
pub use token_parser::token_parser;
mod tree_parser;
pub use tree_parser::tree_parser;
mod use_parser;
pub use use_parser::use_parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Thingie {
    Token(String),
    Tree(Thread, TreesUsed),
}

//! Seq{
//!     a,
//!     b,
//!     c,
//! }
//! (seq{_2,_3,_4}, {_2:a, _3:b, _4:c})
//!

use super::{Thread, TreePool};
use nom::character::complete::multispace0 as parse_space;

mod selector;
pub use selector::parse_selector;
mod sequence;
pub use sequence::parse_sequence;
mod parse_action;
pub use parse_action::parse_action;
mod parse_file;
pub use parse_file::parse_file;
mod parse_token;
pub use parse_token::parse_token;
mod parse_tree;
pub use parse_tree::parse_tree;

type TreesUsed = TreePool;

#[derive(Debug, PartialEq, Eq)]
pub enum Thingie {
    Token(String),
    Tree(Thread, TreesUsed),
}

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
mod parse_combine;
pub use parse_combine::parse_combine;
mod parse_eat;
pub use parse_eat::parse_eat;
mod parse_file;
pub use parse_file::parse_file;
mod parse_ident;
pub use parse_ident::parse_ident;
mod parse_inventory_have_ge;
pub use parse_inventory_have_ge::parse_inventory_have_ge;
mod parse_token;
pub use parse_token::parse_token;
mod parse_tree;
pub use parse_tree::parse_tree;
mod parse_u8;
pub use parse_u8::parse_u8;
mod parse_use;
pub use parse_use::parse_use;

type TreesUsed = TreePool;

#[derive(Debug, PartialEq, Eq)]
pub enum Thingie {
    Token(String),
    Tree(Thread, TreesUsed),
}

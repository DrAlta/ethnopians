use nom::{branch::alt, combinator::map_res, error::ErrorKind, IResult};

use crate::sandbox::bt::{parser::token_parser, Instruction, Thread, TreePool};

use super::lit_parser;

pub fn forth_threadette_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    alt((
        map_res(token_parser, |x| {
            Ok::<(Thread, TreePool), ()>(match x {
                crate::sandbox::bt::parser::Thingie::Token(token) => {
                    (vec![Instruction::ForthCall(token, 0)], TreePool::new())
                }
                crate::sandbox::bt::parser::Thingie::Tree(vec, hash_map) => (vec, hash_map),
            })
        }),
        lit_parser,
    ))(input)
}

use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::bt::{parser::{space_parser, Thingie}, Instruction};

use super::forth_parser;


pub fn forth_tree_parser<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, (body, used), _, _)) = tuple((
        tag("forth"),
        space_parser,
        char('{'),
        space_parser,
        forth_parser::forth_parser,
        space_parser,
        char('}'),
    ))(input)?;
    Ok((
        tail,
        Thingie::Tree(body, used),
    ))
}
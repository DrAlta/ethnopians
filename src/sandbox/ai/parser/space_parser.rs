use nom::{character::complete::multispace0, combinator::opt, error::ErrorKind, sequence::tuple, IResult};

use super::comment_parser;

pub fn space_parser<'a>(input: &'a str) -> IResult<&'a str, (&'a str, Option<((), &'a str)>), (&'a str, ErrorKind)> {
    tuple((
        multispace0,
        opt(tuple((comment_parser, multispace0)))
    ))(input)
}

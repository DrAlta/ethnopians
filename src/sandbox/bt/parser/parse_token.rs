use nom::{character::complete::one_of, combinator::recognize, error::ErrorKind, multi::many1, IResult};

use crate::sandbox::bt::ExecutionToken;

pub fn parse_token<'a>(input: &'a str) -> IResult<&'a str, ExecutionToken, (&'a str, ErrorKind)> {
    let (tail, head) = recognize(many1(one_of("abcdefghijklmnopqrstuzwxyx_1234567890")))(input)?;
    Ok((tail, head.to_owned()))
}

use nom::{character::complete::one_of, combinator::recognize, error::ErrorKind, multi::many1, IResult};

pub fn parse_ident<'a>(input: &'a str) -> IResult<&'a str, &'a str, (&'a str, ErrorKind)> {
    recognize(many1(one_of("abcdefghijklmnopqrstuvwxyz_1234567890")))(input)
}

use nom::{character::complete::char, combinator::opt, error::ErrorKind, multi::many1, IResult};

pub fn parse_space<'a>(input: &'a str) -> IResult<&'a str, (), (&'a str, ErrorKind)> {
    let (tail, _) = opt(many1(char(' ')))(input)?;
    Ok((tail, ()))
}

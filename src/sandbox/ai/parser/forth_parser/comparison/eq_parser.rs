use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::recognize,
    error::ErrorKind, IResult,
};

use crate::sandbox::ai::{Instruction, TaskPool, Thread};

pub fn eq_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = alt((tag("eq"), recognize(char('='))))(input)?;
    Ok((tail, (vec![Instruction::ForthEq], TaskPool::new())))
}

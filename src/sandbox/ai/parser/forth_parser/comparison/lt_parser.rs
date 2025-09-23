use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::recognize,
    error::ErrorKind, IResult,
};

use crate::sandbox::ai::{Instruction, TaskPool, Thread};

pub fn lt_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = alt((tag("lt"), recognize(char('<'))))(input)?;
    Ok((tail, (vec![Instruction::ForthLT], TaskPool::new())))
}

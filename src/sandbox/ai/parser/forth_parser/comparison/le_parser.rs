use nom::{branch::alt, bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TaskPool};

pub fn le_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = alt((tag("le"), tag("<=")))(input)?;
    Ok((tail, (vec![Instruction::ForthLE], TaskPool::new())))
}

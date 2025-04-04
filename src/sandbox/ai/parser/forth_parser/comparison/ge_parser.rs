use nom::{branch::alt, bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TaskPool};

pub fn ge_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = alt((tag("ge"), tag(">=")))(input)?;
    Ok((tail, (vec![Instruction::ForthGE], TaskPool::new())))
}

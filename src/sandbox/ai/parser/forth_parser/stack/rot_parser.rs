use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TaskPool};

pub fn rot_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("rot")(input)?;
    Ok((tail, (vec![Instruction::ForthRot], TaskPool::new())))
}

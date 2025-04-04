use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TaskPool};

pub fn return_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("return")(input)?;
    Ok((tail, (vec![Instruction::ForthReturn], TaskPool::new())))
}

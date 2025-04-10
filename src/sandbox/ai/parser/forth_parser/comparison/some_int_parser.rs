use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, TaskPool, Thread};

pub fn some_int_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("some_int")(input)?;
    Ok((tail, (vec![Instruction::ForthSomeInt], TaskPool::new())))
}

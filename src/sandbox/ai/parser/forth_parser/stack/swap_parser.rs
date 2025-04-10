use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, TaskPool, Thread};

pub fn swap_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("swap")(input)?;
    Ok((tail, (vec![Instruction::ForthSwap], TaskPool::new())))
}

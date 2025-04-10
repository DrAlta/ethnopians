use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, TaskPool, Thread};

pub fn get_location_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("get_location")(input)?;
    Ok((tail, (vec![Instruction::ForthGetLocation], TaskPool::new())))
}

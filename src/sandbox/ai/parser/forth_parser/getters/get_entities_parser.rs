use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, TaskPool, Thread};

pub fn get_entities_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("get_entities")(input)?;
    Ok((tail, (vec![Instruction::ForthGetEntities], TaskPool::new())))
}

use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TreePool};

pub fn get_entities_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("get_entities")(input)?;
    Ok((tail, (vec![Instruction::ForthGetEntities], TreePool::new())))
}

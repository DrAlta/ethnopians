use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TaskPool};

pub fn some_entity_id_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("some_entity_id")(input)?;
    Ok((
        tail,
        (vec![Instruction::ForthSomeEntityId], TaskPool::new()),
    ))
}

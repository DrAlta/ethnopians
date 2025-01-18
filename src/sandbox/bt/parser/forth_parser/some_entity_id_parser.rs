use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::bt::{Instruction, Thread, TreePool};

pub fn some_entity_id_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("some_entity_id")(input)?;
    Ok((tail, (vec![Instruction::ForthSomeEntityId], TreePool::new())))
}

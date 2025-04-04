use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TaskPool};

pub fn set_blackboard<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("set_blackboard")(input)?;
    Ok((
        tail,
        (vec![Instruction::ForthSetBlackboard], TaskPool::new()),
    ))
}

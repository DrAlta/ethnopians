use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TreePool};

pub fn get_blackboard<'a>(input: &'a str) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("get_blackboard")(input)?;
    Ok((tail, (vec![Instruction::ForthGetBlackboard], TreePool::new())))
}

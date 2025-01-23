use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{InpulseId, Instruction, Thread, TreePool};

pub fn take_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("take")(input)?;
    Ok((tail, (vec![Instruction::Action(InpulseId::Take)], TreePool::new())))
}

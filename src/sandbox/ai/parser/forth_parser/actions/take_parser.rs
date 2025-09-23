use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{InpulseId, Instruction, TaskPool, Thread};

pub fn take_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("take")(input)?;
    Ok((
        tail,
        (
            vec![Instruction::ForthAction(InpulseId::Take)],
            TaskPool::new(),
        ),
    ))
}

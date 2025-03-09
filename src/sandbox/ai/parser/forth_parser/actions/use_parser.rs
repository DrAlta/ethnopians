use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{InpulseId, Instruction, Thread, TreePool};

pub fn use_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("use")(input)?;
    Ok((
        tail,
        (vec![
            Instruction::ForthAction(InpulseId::Use),
        ], TreePool::new()),
    ))
}

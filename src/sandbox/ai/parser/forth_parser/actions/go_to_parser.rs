use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{InpulseId, Instruction, Thread, TreePool};

pub fn go_to_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("go_to")(input)?;
    Ok((
        tail,
        (vec![
            Instruction::ForthAction(InpulseId::GoTo),
        ], TreePool::new()),
    ))
}

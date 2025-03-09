use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{InpulseId, Instruction, Thread, TreePool};

pub fn plant_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("plant")(input)?;
    Ok((
        tail,
        (vec![
            Instruction::ForthAction(InpulseId::Plant),
        ], TreePool::new()),
    ))
}

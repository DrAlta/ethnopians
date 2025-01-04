use std::collections::HashMap;

use nom::{branch::alt, bytes::complete::tag, combinator::map_res, error::ErrorKind, IResult};

use crate::sandbox::bt::{InpulseId, Instruction, Thread};

use super::TreesUsed;

pub fn parse_action<'a>(
    input: &'a str,
) -> IResult<
    &'a str,
    (Thread, TreesUsed),
    (&'a str, ErrorKind),
> {
    let (tail, i) = alt((
        map_res(
            tag("act1"),
            |_| {
                Ok::<Instruction,()>(Instruction::Action(InpulseId::Act1))
            }
        ),
        map_res(
            tag("act2"),
            |_| {
                Ok::<Instruction,()>(Instruction::Action(InpulseId::Act2))
            }
        ),
        map_res(
            tag("act3"),
            |_| {
                Ok::<Instruction,()>(Instruction::Action(InpulseId::Act3))
            }
        ),
    ))(input)?;
    Ok((
        tail, (
            i,
            HashMap::new()
        )
    ))
}
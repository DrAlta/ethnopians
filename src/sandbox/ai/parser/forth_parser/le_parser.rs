use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TreePool};

pub fn le_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("le")(input)?;
    Ok((tail, (vec![Instruction::ForthLE], TreePool::new())))
}

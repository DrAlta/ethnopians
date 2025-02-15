use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TreePool};

pub fn drop_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("drop")(input)?;
    Ok((tail, (vec![Instruction::ForthDrop], TreePool::new())))
}

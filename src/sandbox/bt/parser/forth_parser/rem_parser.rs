use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::bt::{Instruction, Thread, TreePool};

pub fn rem_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("rem")(input)?;
    Ok((tail, (vec![Instruction::ForthRem], TreePool::new())))
}

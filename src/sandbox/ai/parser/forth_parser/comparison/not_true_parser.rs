use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TreePool};

pub fn not_true_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("not_true")(input)?;
    Ok((tail, (vec![Instruction::ForthNotTrue], TreePool::new())))
}

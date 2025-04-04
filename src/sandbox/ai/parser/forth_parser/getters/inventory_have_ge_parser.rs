use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TaskPool};

pub fn inventory_have_ge_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("inventory_have_ge")(input)?;
    Ok((tail, (vec![Instruction::ForthInventoryGE], TaskPool::new())))
}

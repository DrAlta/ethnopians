use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TaskPool};

pub fn find_in_inventory_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("find_in_inventory")(input)?;
    Ok((
        tail,
        (vec![Instruction::ForthFindInInventory], TaskPool::new()),
    ))
}

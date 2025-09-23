use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, TaskPool, Thread};

pub fn remove_entities_of_type_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("remove_entities_of_type")(input)?;
    Ok((
        tail,
        (
            vec![Instruction::ForthRemoveEntitiesOfType],
            TaskPool::new(),
        ),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_entities_of_type_test() {
        let source = "remove_entities_of_type";
        let (tail, (head, _pool)) = remove_entities_of_type_parser(source).unwrap();
        assert_eq!(tail, "");
        assert_eq!(head, vec![Instruction::ForthRemoveEntitiesOfType])
    }
}

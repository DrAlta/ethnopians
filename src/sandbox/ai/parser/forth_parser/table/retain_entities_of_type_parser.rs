use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::ai::{Instruction, Thread, TreePool};

pub fn retain_entities_of_type_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("retain_entities_of_type")(input)?;
    Ok((
        tail,
        (
            vec![Instruction::ForthRetainEntitiesOfType],
            TreePool::new(),
        ),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retain_entities_of_type_test() {
        let source = "retain_entities_of_type";
        let (tail, (head, _pool)) = retain_entities_of_type_parser(source).unwrap();
        assert_eq!(tail, "");
        assert_eq!(head, vec![Instruction::ForthRetainEntitiesOfType])
    }
}

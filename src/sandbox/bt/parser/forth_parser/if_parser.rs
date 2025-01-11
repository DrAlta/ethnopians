use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::bt::{
    parser::{forth_parser::forth_parser, space_parser},
    Instruction, Thread, TreePool,
};

pub fn if_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, (thread, used), _, _)) = tuple((
        tag("if"),
        space_parser,
        char('{'),
        space_parser,
        forth_parser,
        space_parser,
        char('}'),
    ))(input)?;
    let mut vec = Vec::new();
    vec.push(Instruction::ForthIf(thread.len()));
    vec.extend(thread.into_iter());
    Ok((tail, (vec, used)))
}

#[cfg(test)]
mod tests {
    use crate::sandbox::bt::StackItem;

    use super::*;

    #[test]
    fn if_parser_test() {
        let input = "if{
            lit(Success)
            return
        }";
        let (tail, (body, used)) = if_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(
            body,
            vec![
                Instruction::ForthIf(2),
                Instruction::ForthLit(StackItem::Success),
                Instruction::ForthReturn,
            ]
        )
    }
}

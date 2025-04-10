use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::ai::{
    parser::{ident_parser, space_parser},
    Instruction, TaskPool, Thread,
};

pub fn jump_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, body, _, _)) = tuple((
        tag("jump"),
        space_parser,
        char('('),
        space_parser,
        ident_parser,
        space_parser,
        char(')'),
    ))(input)?;
    Ok((
        tail,
        (
            vec![Instruction::ForthJump(body.to_owned(), 0)],
            TaskPool::new(),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use crate::sandbox::ai::Instruction;

    use super::*;

    #[test]
    fn jump_test() {
        let input = "jump(one)";
        let (tail, (body, used)) = jump_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TaskPool::new());
        assert_eq!(body, vec![Instruction::ForthJump("one".to_owned(), 0)])
    }
}

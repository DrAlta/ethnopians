use nom::{
    bytes::complete::tag,
    character::streaming::multispace0,
    combinator::{map_res, recognize},
    error::ErrorKind,
    sequence::tuple,
    IResult,
};

use crate::sandbox::bt::{Thread, TreePool};

use super::{balanced::balanced, forth_threadette_parser};

mod r#if;
use r#if::If;

pub fn if_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, body) = balanced(
        map_res(tuple((multispace0, forth_threadette_parser)), |(_, x)| {
            Result::<(Thread, TreePool), (&'a str, ErrorKind)>::Ok(x)
        }),
        recognize(tuple((multispace0, tag("if")))),
        recognize(tuple((multispace0, tag("then")))),
    )(input)?;

    let x = body.flatten();
    Ok((tail, x))
}

#[cfg(test)]
mod tests {
    use crate::sandbox::bt::{Instruction, StackItem};

    use super::*;

    #[test]
    fn if_parser_test() {
        let input = "if
            lit(Success)
            return
        then";
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
    #[test]
    fn nested_if_parser_test() {
        let input = "if
            lit(1)
            if
                return
            then
            lit(2)
        then";
        let (tail, (body, used)) = if_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(
            body,
            vec![
                Instruction::ForthIf(4),
                Instruction::ForthLit(StackItem::Int(1)),
                Instruction::ForthIf(1),
                Instruction::ForthReturn,
                Instruction::ForthLit(StackItem::Int(2)),
            ]
        )
    }
}

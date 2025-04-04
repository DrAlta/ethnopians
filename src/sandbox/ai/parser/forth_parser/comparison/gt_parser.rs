use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::recognize,
    error::ErrorKind, IResult,
};

use crate::sandbox::ai::{Instruction, Thread, TaskPool};

pub fn gt_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, _body) = alt((tag("gt"), recognize(char('>'))))(input)?;
    Ok((tail, (vec![Instruction::ForthGT], TaskPool::new())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gt_parser_test() {
        let input = "gt";
        let (tail, (body, used)) = gt_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TaskPool::new());
        assert_eq!(body, vec![Instruction::ForthGT,])
    }
}

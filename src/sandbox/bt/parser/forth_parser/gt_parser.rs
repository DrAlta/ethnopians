use nom::{bytes::complete::tag, error::ErrorKind, IResult};

use crate::sandbox::bt::{Instruction, Thread, TreePool};

pub fn gt_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, _body) = tag("gt")(input)?;
    Ok((tail, (vec![Instruction::ForthGT], TreePool::new())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gt_parser_test() {
        let input = "gt";
        let (tail, (body, used)) = gt_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(body, vec![Instruction::ForthGT,])
    }
}

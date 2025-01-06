use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult
};

use crate::sandbox::bt::{
    parser::parse_space,
    Instruction,
};

use super::{parse_ident, Thingie};

pub fn parse_combine<'a>(
    input: &'a str,
) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, item_a, _, item_b, _, _)) = tuple((
            tag("combine"),
            parse_space,
            char('('),
            parse_space,
            parse_ident,
            tuple((
                parse_space,
                char(','),
                parse_space
            )),
            parse_ident,
            parse_space,
            char(')'),
        ))(input)?;
    Ok((tail, Thingie::Tree(Instruction::Combine(item_a.to_owned(), item_b.to_owned()), HashMap::new())))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_combine_test() {
        let (_, Thingie::Tree(i, _db)) = parse_combine("combine( stone , stick)").unwrap() else {
            panic!()
        };
        assert_eq!(
            i,
            Instruction::Combine(
                "stone".to_owned(), 
                "stick".to_owned()
            )
        )
    }
}

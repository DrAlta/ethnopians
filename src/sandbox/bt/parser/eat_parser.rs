use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::bt::{parser::space_parser, Instruction};

use super::{ident_parser, Thingie};

pub fn eat_parser<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, item_a, _, _)) = tuple((
        tag("eat"),
        space_parser,
        char('('),
        space_parser,
        ident_parser,
        space_parser,
        char(')'),
    ))(input)?;
    Ok((
        tail,
        Thingie::Tree(Instruction::Eat(item_a.to_owned()), HashMap::new()),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eat_parser_test() {
        let (_, Thingie::Tree(i, _db)) = eat_parser("eat ( stone )").unwrap() else {
            panic!()
        };
        assert_eq!(i, Instruction::Eat("stone".to_owned(),))
    }
}

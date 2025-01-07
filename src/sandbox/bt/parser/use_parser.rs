use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::bt::{parser::space_parser, Instruction};

use super::{ident_parser, Thingie};

pub fn use_parser<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, item_a, _, item_b, _, _)) = tuple((
        tag("use"),
        space_parser,
        char('('),
        space_parser,
        ident_parser,
        tuple((space_parser, char(','), space_parser)),
        ident_parser,
        space_parser,
        char(')'),
    ))(input)?;
    Ok((
        tail,
        Thingie::Tree(
            Instruction::Use(item_a.to_owned(), item_b.to_owned()),
            HashMap::new(),
        ),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_parser_test() {
        let (_, Thingie::Tree(i, _db)) = use_parser("use ( stone , stick)").unwrap() else {
            panic!()
        };
        assert_eq!(i, Instruction::Use("stone".to_owned(), "stick".to_owned()))
    }
}

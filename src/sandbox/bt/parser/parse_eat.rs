use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::bt::{parser::parse_space, Instruction};

use super::{parse_ident, Thingie};

pub fn parse_eat<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, item_a, _, _)) = tuple((
        tag("eat"),
        parse_space,
        char('('),
        parse_space,
        parse_ident,
        parse_space,
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
    fn parse_eat_test() {
        let (_, Thingie::Tree(i, _db)) = parse_eat("eat ( stone )").unwrap() else {
            panic!()
        };
        assert_eq!(i, Instruction::Eat("stone".to_owned(),))
    }
}

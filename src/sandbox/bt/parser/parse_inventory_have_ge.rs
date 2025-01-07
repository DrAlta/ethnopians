use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::bt::{parser::parse_space, Instruction};

use super::{parse_ident, parse_u8, Thingie};

pub fn parse_inventory_have_ge<'a>(
    input: &'a str,
) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, item, _, number, _, _)) = tuple((
        tag("inventory_have_ge"),
        parse_space,
        char('('),
        parse_space,
        parse_ident,
        tuple((parse_space, char(','), parse_space)),
        parse_u8,
        parse_space,
        char(')'),
    ))(input)?;
    Ok((
        tail,
        Thingie::Tree(
            Instruction::InventoryGE(item.to_owned(), number),
            HashMap::new(),
        ),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_inventory_have_ge_test() {
        let (_, Thingie::Tree(i, _db)) =
            parse_inventory_have_ge("inventory_have_ge ( stone ,1 )").unwrap()
        else {
            panic!()
        };
        assert_eq!(i, Instruction::InventoryGE("stone".to_owned(), 1))
    }
}

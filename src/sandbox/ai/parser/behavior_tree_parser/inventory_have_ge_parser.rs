use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::ai::{
    parser::{behavior_tree_parser::Thingie, i32_parser, ident_parser, space_parser},
    Instruction,
};

pub fn inventory_have_ge_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, item, _, number, _, _)) = tuple((
        tag("inventory_have_ge"),
        space_parser,
        char('('),
        space_parser,
        ident_parser,
        tuple((space_parser, char(','), space_parser)),
        i32_parser,
        space_parser,
        char(')'),
    ))(input)?;
    Ok((
        tail,
        Thingie::Tree(
            vec![Instruction::InventoryGE(item.to_owned(), number)],
            HashMap::new(),
        ),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_have_ge_parser_test() {
        let (_, Thingie::Tree(i, _db)) =
            inventory_have_ge_parser("inventory_have_ge ( stone ,1 )").unwrap()
        else {
            panic!()
        };
        assert_eq!(i, vec![Instruction::InventoryGE("stone".to_owned(), 1)])
    }
}

use std::collections::BTreeMap;

use crate::sandbox::ai::{
    parser::{
        behavior_tree_parser::{tree_parser, Thingie},
        ident_parser, space_parser,
    },
    BlackboardKey, BlackboardValue, Instruction, TaskPool, Variable,
};

use nom::{
    bytes::complete::tag,
    character::{complete::char, streaming::multispace0},
    error::ErrorKind,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn blackboard_parser<'a, 'b>(
    input: &'a str,
) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let mut hash = TaskPool::new();
    //                1  2  3  4  5       6  7  8  9 10, 11,  12 13
    let (tail, (_, _, _, _, values, _, _, _, _, _, tree, _, _)) = //map_res(
        tuple((
            tag("blackboard"),//01
            space_parser,//02
            char('('),//03
            space_parser,//04
            separated_list1(//05
                tuple((
                    multispace0,
                    char(','),
                    multispace0,
                )),
                tuple((
                    ident_parser,
                    tuple((
                        multispace0,
                        tag("=>"),
                        multispace0,
                    )),
                    ident_parser,
                ))
            ),
            space_parser,//06
            char(')'),//07
            space_parser, //08
            char('{'), //09
            space_parser, //10
            separated_list1( //11
                tuple((
                    space_parser,
                    char(','),
                    space_parser
                )),
                tree_parser
            ),
            space_parser,//12
            char('}'),//13
        ))(input)?;
    let _: BTreeMap<BlackboardKey, Variable<BlackboardKey, BlackboardValue>> = values
        .into_iter()
        .map(|(k, _, v)| (k.to_owned(), Variable::Defer(v.to_owned())))
        .collect();

    let mut vec = Vec::new();
    for (idx, thingie) in tree.into_iter().enumerate() {
        match thingie {
            Thingie::Token(token) => vec.push(token),
            Thingie::Tree(mut this_i, db) => {
                let thread_name = format!("@{}", idx + 1);
                for (k, mut v) in db.into_iter() {
                    v.iter_mut().for_each(|x| x.correct(&thread_name));
                    assert_eq!(hash.insert(format!("{thread_name}{k}"), v), None,);
                }
                vec.push(thread_name.clone());
                this_i.iter_mut().for_each(|x| x.correct(&thread_name));
                hash.insert(thread_name, this_i);
            }
        }
    }
    Ok((tail, Thingie::Tree(vec![Instruction::Selector(vec)], hash)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blackboard_parser_test() {
        let (tail, Thingie::Tree(_i, _db)) = blackboard_parser(
            "blackboard ( a => one , b => two ) {use(hands, tree), use(tree, hands)}",
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(tail, "");
        // assert_eq!(i, vec![Instruction::InventoryGE("stone".to_owned(), 1)])
    }
}

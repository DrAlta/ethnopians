use std::collections::HashMap;

use nom::{
    character::complete::char, combinator::map_res, error::ErrorKind, multi::separated_list1,
    sequence::tuple, IResult,
};

use crate::sandbox::bt::{
    parser::{ident_parser, space_parser, tree_parser, Thingie, TreesUsed},
    ExecutionToken, Instruction, TreePool,
};

pub fn file_parser<'a>(
    input: &'a str,
    //    _prefix: &'b str
) -> IResult<&'a str, TreePool, (&'a str, ErrorKind)> {
    //    let mut hash = HashMap::new();
    let (tail, (_, head, _)) = tuple((
        space_parser,
        separated_list1(
            tuple((space_parser, char(';'), space_parser)),
            named_tree_parser,
        ),
        space_parser,
    ))(input)?;
    let mut hash = HashMap::new();
    for (_thread_name, body) in head {
        hash.extend(body.into_iter());
    }
    Ok((tail, hash))
}
/// named_tree_parser() addes the tree to the TreePool
pub fn named_tree_parser<'a>(
    input: &'a str,
    //    _prefix: &'b str
) -> IResult<&'a str, (ExecutionToken, TreePool), (&'a str, ErrorKind)> {
    //    let mut hash = HashMap::new();
    let (tail, (thread_name, _, _, _, (mut i, db))) = tuple((
        ident_parser,
        space_parser,
        char('='),
        space_parser,
        map_res(tree_parser, |x| {
            let Thingie::Tree(i, used) = x else {
                return Err(()).into();
            };
            Ok::<(Instruction, TreesUsed), ()>((i, used))
        }),
    ))(input)?;
    let mut hash = HashMap::new();
    for (k, mut v) in db.into_iter() {
        v.correct(thread_name);
        assert_eq!(hash.insert(format!("{thread_name}{k}"), v), None,);
    }
    i.correct(thread_name);
    hash.insert(thread_name.to_owned(), i);
    Ok((tail, (thread_name.to_owned(), hash)))
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;


    #[test]
    fn named_tree_parser_test() {
        let source = r#"have_2_wood_2 = sel{
        inventory_have_ge(wood, 2),
        have_axe,
        go_to_tree,
        use(axe, tree)
    }"#;
        let (tail, db) = file_parser(source).unwrap();
    let standard =         HashMap::from([
        (
            "have_2_wood_2".to_owned(),
            Instruction::Selector(vec![
                "have_2_wood_2_1".to_owned(),
                "have_axe".to_owned(),
                "go_to_tree".to_owned(),
                "have_2_wood_2_4".to_owned(),
            ]),
        ),
        (
            "have_2_wood_2_1".to_owned(),
            Instruction::InventoryGE("wood".to_owned(), 2)
        ),
        (
            "have_2_wood_2_4".to_owned(),
            Instruction::Use("axe".to_owned(), "tree".to_owned())
        )
    ]);
        assert_eq!(
            db,
            standard
        );
        assert_eq!(tail, "");
    }
    #[test]
    fn named_tree_parser_test2() {
        let source = r#"sat_hunger = selector{
        sel{
            selector{
                inventory_have_ge(veg, 1)
            }
        }
    }"#;
        let (tail, db) = file_parser(source).unwrap();
        let standard = BTreeMap::from([
            (
                "sat_hunger".to_owned(),
                Instruction::Selector(vec![
                    "sat_hunger_1".to_owned(),
                ]),
            ),
            (
                "sat_hunger_1".to_owned(),
                Instruction::Selector(vec![
                    "sat_hunger_1_1".to_owned(),
                ])
            ),
            (
                "sat_hunger_1_1".to_owned(),
                Instruction::Selector(vec![
                    "sat_hunger_1_1_1".to_owned(),
                ])
            ),
            (
                "sat_hunger_1_1_1".to_owned(),
                Instruction::InventoryGE("veg".to_owned(), 1)
            )
        ]);
        assert_eq!(
            standard,
            db.into_iter().collect(),
        );
        assert_eq!(tail, "");
    }
    /*
    #[test]
    fn named_tree_parser_test2() {
        let source = r#"sat_hunger = selector{
        dont_need_to_eat,
        seq{
            selector{
                inventory_have_ge(veg, 1),
                get_veg
            },
            eat(veg)
        }
    }"#;
        let (tail, db) = file_parser(source).unwrap();
        let standard = BTreeMap::from([
            (
                "sat_hunger".to_owned(),
                Instruction::Selector(vec![
                    "dont_need_to_eat".to_owned(),
                    "sat_hunger_2".to_owned(),
                ]),
            ),
            (
                "sat_hunger_2".to_owned(),
                Instruction::Sequence(vec![
                    "sat_hunger_2_1".to_owned(),
                    "sat_hunger_2_2".to_owned(),
                ])
            ),
            (
                "sat_hunger_2_1".to_owned(),
                Instruction::Selector(vec![
                    "sat_hunger_2_1_1".to_owned(),
                    "get_veg".to_owned(),
                ])
            ),
            (
                "sat_hunger_2_2".to_owned(),
                Instruction::Eat("veg".to_owned())
            )
        ]);
        assert_eq!(
            standard,
            db.into_iter().collect(),
        );
        assert_eq!(tail, "");
    }
    */
}
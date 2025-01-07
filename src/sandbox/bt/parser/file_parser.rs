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
    let (tail, (thread_name, _, _, _, (i, db))) = tuple((
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
    hash.insert(thread_name.to_owned(), i);
    Ok((tail, (thread_name.to_owned(), hash)))
}

#[test]
fn foo() {
    let source = r#"
have_2_stone_2 = sel{
    inventory_have_ge(stone, 2),
    seq{
        go_to_stone,
        take_stone
    }
};
have_2_stone = seq{
    have_2_stone_2,
    have_2_stone_2
};
have_knife = sel{
    inventory_have_ge(knife, 1), 
    seq{
        have_2_stone,
        combine(stone, stone)
    }
};
have_stick = sel{
    inventory_have_ge(stick, 1), 
    seq{
        go_to_tree,
        use(hands, tree)
    }
};
have_axe = sel{
    inventory_have_ge(axe, 1),
    seq{
        have_knife,
        have_stick,
        combine(stick, knife)
    }
};
have_2_wood_2 = sel{
    inventory_have_ge(wood, 2),
    have_axe,
    go_to_tree,
    use(axe, tree)
};
have_2_wood =seq{
    have_2_wood_2,
    have_2_wood_2
};
have_house = sel {
    is_house_in_range,
    seq{
        have_2_wood,
        combine(wood,wood)
    }
};
sat_hunger = selector{
    dont_need_to_eat,
    seq{
        selector{
            inventory_have_ge(veg, 1),
            get_veg
        },
        eat(veg)
    }
}
"#;
    let (tail, db) = file_parser(source).unwrap();
    println!("{db:?}");
    assert_eq!(tail, "");
}

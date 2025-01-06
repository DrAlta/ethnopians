use std::collections::HashMap;

use nom::{character::complete::{char, one_of}, combinator::{map_res, recognize}, error::ErrorKind, multi::{many1, separated_list1}, sequence::tuple, IResult};

use crate::sandbox::bt::{ExecutionToken, Instruction, TreePool, parser::{parse_space, parse_tree, Thingie, TreesUsed}};


pub fn parse_file<'a>(
    input: &'a str,
    //    _prefix: &'b str
) -> IResult<&'a str, TreePool, (&'a str, ErrorKind)> {
    //    let mut hash = HashMap::new();
    let (tail, (_, head, _)) = tuple((
        parse_space,
        separated_list1(
                tuple((
                    parse_space,
                    char(';'),
                    parse_space,
                )),
                parse_named_tree
        ),
        parse_space,
    ))(input)?;
    let mut hash = HashMap::new();
    for (_thread_name, body) in head {
        hash.extend(body.into_iter());
    };
    Ok(
        (
            tail,
            hash,
        )
    )
 
}
/// parse_named_tree() addes the tree to the TreePool
pub fn parse_named_tree<'a>(
    input: &'a str,
    //    _prefix: &'b str
) -> IResult<&'a str, (ExecutionToken, TreePool), (&'a str, ErrorKind)> {
    //    let mut hash = HashMap::new();
    let (tail,(thread_name, _ ,_ ,_ , (i, db))) = tuple((
        recognize(many1(one_of("abcdefghijklmnopqrstuzwxyx_1234567890"))),
        parse_space,
        char('='),
        parse_space,
        map_res(
            parse_tree,
            |x| {
                let Thingie::Tree(i, used) = x else {
                    return Err(()).into()
                };
                Ok::<(Instruction, TreesUsed), ()>((i, used))
            }
        )
    ))(input)?;
    let mut hash = HashMap::new();
    for (k, mut v) in db.into_iter() {
        v.correct(thread_name);
        assert_eq!(hash.insert(format!("{thread_name}{k}"), v), None,);
    }
    hash.insert(thread_name.to_owned(), i);
    Ok((
        tail,
        (
            thread_name.to_owned(),
            hash
        )
    ))
    
}

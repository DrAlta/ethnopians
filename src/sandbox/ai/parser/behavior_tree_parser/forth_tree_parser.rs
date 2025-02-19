use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::ai::{
    parser::{behavior_tree_parser::Thingie, forth_parser, space_parser},
    Instruction,
};

pub fn forth_tree_parser<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, (mut i, db), _, _)) = tuple((
        tag("forth"),
        space_parser,
        char('{'),
        space_parser,
        forth_parser::forth_parser,
        space_parser,
        char('}'),
    ))(input)?;
    //    Ok((tail, Thingie::Tree(body, used)))
    //vvv new vvv
    let thread_name = "_0";
    let mut hash = HashMap::new();
    for (k, mut v) in db.into_iter() {
        v.iter_mut().for_each(|x| x.correct(thread_name));
        assert_eq!(hash.insert(format!("{k}"), v), None,);
    }
    i.iter_mut().for_each(|x| x.correct(thread_name));
    hash.insert(thread_name.to_owned(), i);
    Ok((
        tail,
        Thingie::Tree(vec![Instruction::ForthTree(thread_name.to_owned())], hash),
    ))
}
#[cfg(test)]
mod tests {
    use crate::sandbox::ai::{Instruction, StackItem};

    use super::*;

    #[test]
    fn forth_tree_parser_test() {
        let input = "forth {
            lit(\"self\")
            get_energy
            is_int
            if
                lit(5)
                gt
                if
                    lit(Success)
                    return
                then
            then
            lit(Failure)
            return
            
        }";
        /*"forth {
        lit(Success)
        }";*/
        let (tail, Thingie::Tree(body, used)) = forth_tree_parser(input).unwrap() else {
            panic!("parser didn't return a THingie::Tree")
        };
        assert_eq!(tail, "");
        assert_eq!(body, vec![Instruction::ForthTree("_0".to_owned())]);
        assert_eq!(
            used.get("_0").unwrap(),
            &vec![
                Instruction::ForthLit(StackItem::String("self".to_owned())),
                Instruction::ForthGetEnergy,
                Instruction::ForthIsInt,
                Instruction::ForthIf(5),
                Instruction::ForthLit(StackItem::Int(5)),
                Instruction::ForthGT,
                Instruction::ForthIf(2),
                Instruction::ForthLit(StackItem::success()),
                Instruction::ForthReturn,
                Instruction::ForthLit(StackItem::failure()),
                Instruction::ForthReturn,
            ]
        )
    }
}

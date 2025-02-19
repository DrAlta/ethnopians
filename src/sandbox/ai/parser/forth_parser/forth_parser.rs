use std::collections::HashMap;

use crate::sandbox::ai::{Corrent, Thread, TreePool};
use nom::{
    character::complete::{multispace0, multispace1},
    error::ErrorKind,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use super::forth_threadette_parser;

pub fn forth_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, (body, _)) = tuple((
        separated_list1(multispace1, forth_threadette_parser),
        multispace0,
    ))(input)?;
    let mut thread = Vec::new();
    let mut pool = HashMap::new();
    for (idx, (mut vec, mut hash_map)) in body.into_iter().enumerate() {
        let prefix = format!("_{idx}");
        hash_map.correct(&prefix);
        vec.iter_mut().for_each(|x| x.correct(&prefix));
        thread.append(&mut vec);
        pool.extend(hash_map.into_iter());
    }
    Ok((tail, (thread, pool)))
}
#[cfg(test)]
mod tests {
    use crate::sandbox::ai::{Instruction, StackItem};

    use super::*;

    #[test]
    fn forth_parser_test() {
        let input = "lit(\"self\")
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
    return";
        let (tail, (body, used)) = forth_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(
            body,
            vec![
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

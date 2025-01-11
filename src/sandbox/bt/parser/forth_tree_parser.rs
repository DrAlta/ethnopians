use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::bt::parser::{space_parser, Thingie};

use super::forth_parser;

pub fn forth_tree_parser<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, (body, used), _, _)) = tuple((
        tag("forth"),
        space_parser,
        char('{'),
        space_parser,
        forth_parser::forth_parser,
        space_parser,
        char('}'),
    ))(input)?;
    Ok((tail, Thingie::Tree(body, used)))
}
#[cfg(test)]
mod tests {
    use crate::sandbox::bt::{Instruction, StackItem, TreePool};

    use super::*;

    #[test]
    fn forth_tree_parser_test() {
        let input = "forth {
            get_energy(self)
            is_int
            if{
                lit(5)
                gt
                if{
                    lit(Success)
                    return
                }
            }
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
        assert_eq!(used, TreePool::new());
        assert_eq!(
            body,
            vec![
                Instruction::ForthGetEnergy("self".to_owned()),
                Instruction::ForthIsInt,
                Instruction::ForthIf(5),
                Instruction::ForthLit(StackItem::Int(5)),
                Instruction::ForthGT,
                Instruction::ForthIf(2),
                Instruction::ForthLit(StackItem::Success),
                Instruction::ForthReturn,
                Instruction::ForthLit(StackItem::Failure),
                Instruction::ForthReturn,
            ]
        )
    }
}



use crate::sandbox::ai::{parser::{comment_parser, space_parser}, Corrent, Instruction, TaskPool, Thread, ThreadName
};
use nom::{
    character::complete::{multispace0, multispace1},
    combinator::opt,
    error::{ErrorKind, ParseError},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub struct Parser{
    this:String, 
    number_of_ifs: u8
}
pub enum Word {
    Lit(i32),
    If,
    Then,
    Else,
    Jump(ThreadName),
    Exit,
}


pub fn word_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, Word, (&'a str, ErrorKind)> {
    todo!()
}


pub fn forth_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let mut stack = vec![Parser{ this: String::new(), number_of_ifs: 0 }];
    let mut thread = Vec::new();
    let mut pool = TaskPool::new();
    loop {
        let Ok((tail, (_, _, word))) = tuple((
            multispace1, 
            opt(
                tuple(
                    (comment_parser, multispace0)
                )
            ),
            word_parser
        ))(input)else {
            todo!("handle finilizing the compilation")
        };
        match word {
            Word::Lit(x) => {
                thread.push(Instruction::ForthLit(crate::sandbox::ai::StackItem::Int(x)))
            },
            Word::If => {
                let Some( Parser{ this, number_of_ifs }) = stack.last_mut() else {
                    return Err(nom::Err::Error((tail, ErrorKind::Fail)));
                };
                *number_of_ifs += 1;
                let then_id =  format!("{this}@if{number_of_ifs}@then");
                assert!(false);//thread.push(Instruction::ForthIf(then_id.clone()));
                stack.push(Parser{ this: then_id, number_of_ifs: 0 })
            },
            Word::Then => {
                let Some( Parser{ this, number_of_ifs }) = stack.pop() else {
                    return Err(nom::Err::Error((tail, ErrorKind::Fail)));
                };
                thread.push(Instruction::ForthCall(this.clone(), 0));
                pool.insert(this, thread);
                thread = Vec::new();
            }
            Word::Else => {
                let Some( Parser{ this, number_of_ifs }) = stack.get(stack.len() -1) else {
                    return Err(nom::Err::Error((tail, ErrorKind::Fail)));
                };
                let else_id = format!("{this}@if{number_of_ifs}@else");
                let Some( Parser{ this, number_of_ifs }) = stack.pop() else {
                    return Err(nom::Err::Error((tail, ErrorKind::Fail)));
                };
                thread.push(Instruction::ForthCall(else_id.clone(), 0));
                pool.insert(this, thread);
                thread = Vec::new();
                stack.push(Parser { this: else_id, number_of_ifs: 0 })


            },
            Word::Jump(_) => todo!(),
            Word::Exit => todo!(),
        }
    }
/* this code from the privious vrsion left here to help figure out how this function fits into the larger program
    for (idx, (mut vec, mut hash_map)) in body.into_iter().enumerate() {
        let prefix = format!("@{idx}");
        hash_map.correct(&prefix);
        vec.iter_mut().for_each(|x| x.correct(&prefix));
        thread.append(&mut vec);
        pool.extend(hash_map.into_iter());
    }
    Ok((tail, (thread, pool)))
*/
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
        assert_eq!(used, TaskPool::new());
        assert_eq!(
            body,
            vec![
                Instruction::ForthLit("self".into()),
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
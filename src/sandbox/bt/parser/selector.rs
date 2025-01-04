use std::collections::HashMap;

use nom::{bytes::complete::tag, character::complete::char, error::ErrorKind, multi::separated_list1, sequence::tuple, IResult};

use crate::sandbox::bt::{parser::{parse_space, parse_tree}, Instruction, Thread};

use super::TreesUsed;

pub fn parse_selector<'a, 'b>(
    input: &'a str,
    //    bt: &'b HashMap<ExecutionToken, Vec::<Instruction>>,
) -> IResult<&'a str, (Thread, TreesUsed), (&'a str, ErrorKind)> {
    let mut hash = HashMap::new();
    let (tail, (_, _, head, _, _)) = //map_res(
        tuple((
            tag("sel{"),
            parse_space,
            separated_list1(
                tuple((
                    parse_space,
                    char(','),
                    parse_space
                )), 
                parse_tree
            ),
            parse_space,
            char('}'),
        ))/*,

        |(_, _, head, _, _)| {
            

        }
    )*/
    (input)?;
    let mut vec = Vec::new();
    for (idx, (i, db)) in head.into_iter().enumerate() {
        let thread_name = format!("_{}", idx + 2);
        for (k, mut v) in db.into_iter() {
            v.correct(&thread_name);
            assert_eq!(
                hash.insert(format!("{thread_name}{k}"), v),
                None,
            );
        }
        vec.push(thread_name.clone());
        hash.insert(thread_name, i);
    };
    Ok((
        tail,
        (
            Instruction::Selector(vec),
            hash
        )
    ))
}


#[cfg(test)]
mod tests {
    use crate::sandbox::bt::InpulseId;

    use super::*;

    #[test]
    fn selector_acts_test(){
        let (_, (i, db)) = parse_selector("seq{act1, act2, act3}").unwrap();
        assert_eq!(
            i,
            Instruction::Selector(vec!["_2".to_owned(), "_3".to_owned(), "_4".to_owned()]),
        );
        assert_eq!(
            db,
            HashMap::from([
                ("_2".to_owned(), Instruction::Action(InpulseId::Act1)),
                ("_3".to_owned(), Instruction::Action(InpulseId::Act2)),
                ("_4".to_owned(), Instruction::Action(InpulseId::Act3)),
            ])
        );
    }
    
}
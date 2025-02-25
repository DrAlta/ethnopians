use nom::{
    branch::alt,
    combinator::{eof, map_res},
    error::ErrorKind,
    sequence::tuple,
    IResult,
};
use qol::logy;

use crate::sandbox::ai::{
    parser::{
        forth_parser::{
            add_parser, distance_parser, div_parser, dup_parser, eq_parser, find_nearest_parser,
            ge_parser, get_blackboard, get_energy_parser, get_entities_parser, get_hp_parser,
            get_location_parser, go_to_parser, gt_parser, if_parser, is_int_parser, le_parser,
            lit_parser, lt_parser, mul_parser, rem_parser, remove_entities_of_type_parser,
            return_parser, some_coord_parser, some_entity_id_parser, some_int_parser, sub_parser,
            swap_parser, take_parser,
        },
        ident_parser,
    },
    Instruction, Thread, TreePool,
};

use super::{drop_parser, getters::set_blackboard, is_empty_parser, jump_parser, pop_last_parser};

pub fn forth_threadette_parser_2<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    alt((
        is_empty_parser,
        remove_entities_of_type_parser, // this needs to be before `rem_parser`
        lit_parser,
        jump_parser,
        //calc
        distance_parser,
        // athirthmatic
        add_parser,
        sub_parser,
        mul_parser,
        div_parser,
        rem_parser,
        // getters
        alt((
            find_nearest_parser,
            get_blackboard,
            get_entities_parser,
            get_energy_parser,
            get_hp_parser,
            get_location_parser,
            pop_last_parser,
            set_blackboard,
        )),
        // comparisions
        alt((
            eq_parser,
            ge_parser,
            gt_parser,
            le_parser,
            lt_parser,
            is_int_parser,
            some_coord_parser,
            some_entity_id_parser,
            some_int_parser,
        )),
        //flow
        if_parser,
        return_parser,
        //stack manip
        drop_parser,
        dup_parser,
        swap_parser,
        // actions
        alt((go_to_parser, take_parser)),
    ))(input)
}

pub fn forth_threadette_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    alt((
        map_res(ident_parser, |x| {
            /*
            #[cfg(test)]
            println!("found: {x:?}");
            */
            // these 3 need to be handled special be couse they cover more that a single ident
            match x {
                "then" => {
                    logy!("error", "we shouldn't find a 'then'");
                    return Err(());
                }
                "if" | "lit" | "jump" => return Err(()),
                _ => (),
            };
            let a = tuple((forth_threadette_parser_2, eof))(x);
            if let Ok(("", (b, _))) = a {
                return Ok::<(Thread, TreePool), ()>(b);
            } else {
                logy!("trace-parser-threadette", "got was {a:?}");
                Ok((
                    vec![Instruction::ForthCall(x.to_owned(), 0)],
                    TreePool::new(),
                ))
            }
        }),
        forth_threadette_parser_2,
    ))(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_test() {
        let source = "gfoo";
        let (tail, (head, _pool)) = forth_threadette_parser(source).unwrap();
        assert_eq!(tail, "");
        assert_eq!(head, vec![Instruction::ForthCall("gfoo".to_owned(), 0)])
    }
    #[test]
    fn jump_test() {
        let source = "jump(to_here)";
        let (tail, (head, _pool)) = forth_threadette_parser(source).unwrap();
        assert_eq!(tail, "");
        assert_eq!(head, vec![Instruction::ForthJump("to_here".to_owned(), 0)])
    }
    #[test]
    fn remove_entities_of_type_test() {
        let source = "remove_entities_of_type";
        let (tail, (head, _pool)) = forth_threadette_parser(source).unwrap();
        assert_eq!(tail, "");
        assert_eq!(head, vec![Instruction::ForthRemoveEntitiesOfType])
    }
}

use nom::{branch::alt, combinator::map_res, error::ErrorKind, IResult};

use crate::sandbox::ai::{
    parser::{
        behavior_tree_parser::{token_parser, Thingie},
        forth_parser::{
            add_parser, distance_parser, div_parser, dup_parser, eq_parser, find_nearest_parser,
            ge_parser, get_blackboard, get_energy_parser, get_hp_parser, get_location_parser,
            go_to_parser, gt_parser, if_parser, is_int_parser, le_parser, lit_parser, lt_parser,
            mul_parser, rem_parser, return_parser, some_coord_parser, some_entity_id_parser,
            some_int_parser, sub_parser, swap_parser, take_parser,
        },
    },
    Instruction, Thread, TreePool,
};

pub fn forth_threadette_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    alt((
        lit_parser,
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
            get_energy_parser,
            get_hp_parser,
            get_location_parser,
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
        dup_parser,
        swap_parser,
        // actions
        //      alt((
        go_to_parser,
        take_parser,
        //        )),
        // function calls, this needs to be last so as not to gobble the other tags
        map_res(token_parser, |x| {
            Ok::<(Thread, TreePool), ()>(match x {
                Thingie::Token(token) => (vec![Instruction::ForthCall(token, 0)], TreePool::new()),
                Thingie::Tree(vec, hash_map) => (vec, hash_map),
            })
        }),
    ))(input)
}

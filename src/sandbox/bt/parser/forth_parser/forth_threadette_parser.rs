use nom::{branch::alt, combinator::map_res, error::ErrorKind, IResult};

use crate::sandbox::bt::{
    parser::{
        forth_parser::{get_energy_parser, get_hp_parser, lit_parser},
        token_parser,
    },
    Instruction, Thread, TreePool,
};

use super::{
    add_parser, div_parser, ge_parser, gt_parser, if_parser, is_int_parser, le_parser, lt_parser,
    mul_parser, rem_parser, return_parser, sub_parser,
};

pub fn forth_threadette_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    alt((
        lit_parser,
        // athirthmatic
        add_parser,
        sub_parser,
        mul_parser,
        div_parser,
        rem_parser,
        // getters
        get_hp_parser,
        get_energy_parser,
        // comparisions
        ge_parser,
        gt_parser,
        le_parser,
        lt_parser,
        is_int_parser,
        //flow
        if_parser,
        return_parser,
        // function calls, this needs to be last so as not to gobble the other tags
        map_res(token_parser, |x| {
            Ok::<(Thread, TreePool), ()>(match x {
                crate::sandbox::bt::parser::Thingie::Token(token) => {
                    (vec![Instruction::ForthCall(token, 0)], TreePool::new())
                }
                crate::sandbox::bt::parser::Thingie::Tree(vec, hash_map) => (vec, hash_map),
            })
        }),
    ))(input)
}
use crate::sandbox::ai::{parser::file_parser, TreePool};

pub fn get_hermit_behavoir_tree() -> TreePool {
    let source = r#"
have_2_stone_02 = sel{
    inventory_have_ge(stone, 2),
    seq{
        go_to_stone,
        take_stone
    }
};
have_2_stone = seq{
    have_2_stone_02,
    have_2_stone_02
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
have_2_wood_02 = sel{
    inventory_have_ge(wood, 2),
    have_axe,
    go_to_tree,
    use(axe, tree)
};
have_2_wood =seq{
    have_2_wood_02,
    have_2_wood_02
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
    blackboard(food => veg) {
        seq{
            selector{
                inventory_have_ge(food, 1),
                get_veg
            },
            eat(food)
        }
    }
};
dont_need_to_eat = forth {
    lit("self")
    get_blackboard
    some_entity_id
    if
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
    then
    lit(Failure)
    return
    
};
is_house_in_range = forth{
    lit("self")
    get_blackboard
    some_entity_id
    if
        get_location
        some_coord
        if
            dup
            lit("house")
            find_nearest
            some_entity_id
            if
                get_location
                some_coord
                if
                    distance
                    lit(1000)
                    le
                    if
                        lit(Success)
                        return
                    then
                then
            then
        then
    then
    lit(Failure)
    return
};
get_veg = selector {
    blackboard(food => veg) {
        inventory_have_ge(food, 1),
        forth {
            lit("self")
            get_blackboard
            some_entity_id
            if
                lit("veg")
                find_nearest
                some_entity_id
                if
                    dup
                    get_location
                    some_coord
                    if
                        go_to
                        take
                    then
                then
            then
            lit(Failure)
            return
        }
    }
}"#;
/*
eat_veg = forth {
    lit("self")
    get_blackboard
    some_entity_id
    if
        lit("veg")
        find_inventory
        some_entity_id
        if
            eat
        then
    then
    lit(failure)
    return
};
*/
    let (tail, db) = file_parser(source).unwrap();
    assert_eq!(tail, "");
    db
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::sandbox::ai::{parser::named_tree_parser, Instruction, StackItem};

    use super::*;

    #[test]
    fn hermit_test() {
        let input = "dont_need_to_eat = forth {
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
        let (tail, (_name, body)) = named_tree_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(
            body,
            TreePool::from([(
                "dont_need_to_eat".to_owned(),
                vec![
                    Instruction::ForthLit(StackItem::String("self".to_owned())),
                    Instruction::ForthGetEnergy,
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
            )])
        );
    }

    #[test]
    pub fn check_for_missing_threads_in_hermit_ai() {
        let bt = get_hermit_behavoir_tree();
        let mut missing = HashMap::new();
        for (thread_name, thread) in &bt {
            for i in thread {
                let x = i.missing_threads_used(&bt);
                if !x.is_empty() {
                    missing.insert(thread_name, x);
                }
            }
        }
        for (a, b) in &missing {
            println!("{a}:{b:?}");
        }
        assert_eq!(missing, HashMap::new())
    }
}

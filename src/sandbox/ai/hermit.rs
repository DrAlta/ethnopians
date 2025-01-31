use crate::sandbox::ai::{parser::file_parser, TreePool};

pub fn get_hermit_behavoir_tree() -> TreePool {
    let root = {r#"hermit = sel{
    sat_hunger,
    sat_sleep,
    have_house,
    have_garden,
    harvest_veg,
    plant_veg
}"#};
    let hermit = {r#"sat_hunger = selector{
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
sat_sleep = forth {
    lit("self")
    get_blackboard
    some_entity_id
    if
        get_hp
        some_int
        if
            lit(50)
            lt
            if
                lit("house")
                find_nearest
                some_entity_id
                if
                    dup
                    get_location
                    some_coord
                    if
                        go_to
                        return
                    then
                then
                lit(Failure)
                return
            then
            lit(Success)
            return
        then
    then
    lit(Failure)
    return
};
have_house = sel {
    is_house_in_range,
    seq{
        have_2_wood,
        combine(wood,wood)
    }
};
have_garden = todo;
harvest_veg = todo;
plant_veg = todo"#};
    let sat_hunger= {r#"dont_need_to_eat = forth {
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
                        lit(Success)
                        eq
                        if
                            take
                            return
                        then
                    then
                then
            then
            lit(Failure)
            return
        }
    }
}"#};
    let have_house = {r#"is_house_in_range = forth{
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
have_2_wood =seq{
    have_2_wood_02,
    have_2_wood_02
}"#};
    let have_2_wood = {r#"have_2_wood_02 = sel{
    inventory_have_ge(wood, 2),
    have_axe,
    go_to_tree,
    use(axe, tree)
}"#};
    let have_2_wood_02 = {r#"have_axe = sel{
    inventory_have_ge(axe, 1),
    seq{
        have_knife,
        have_stick,
        combine(stick, knife)
    }
};
go_to_tree = forth {
    lit("self")
    get_blackboard
    some_entity_id
    if
        get_location
        some_coord
        if
            lit("tree")
            find_nearest
            some_entity_id
            if
                get_location
                some_coord
                if
                    go_to
                    return
                then
            then
        then
    then
    lit(Failure)
    return
}"#};
    let have_axe = {r#"have_knife = sel{
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
}"#};
    let have_knife = {r#"have_2_stone = seq{
    have_2_stone_02,
    have_2_stone_02
}"#};
    let have_2_stone = {r#"have_2_stone_02 = sel{
    inventory_have_ge(stone, 2),
    forth {
        lit("self")
        get_blackboard
        some_entity_id
        if
            get_location
            some_coord
            if
                lit("stone")
                find_nearest
                some_entity_id
                if
                    dup
                    some_coord
                    if
                        go_to
                        lit(Success)
                        eq
                        if
                            take
                            return
                        then
                    then
                then
            then
        then
        lit(Failure)
        return
    }
}"#};
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
    let (tail1, mut db) = file_parser(root).unwrap();
    assert_eq!(tail1, "");

    for (idx, source) in [
        hermit,
        sat_hunger,
        have_house,
        have_2_wood,
        have_2_wood_02,
        have_axe,
        have_knife,
        have_2_stone,
    ].into_iter().enumerate() {
        let (tail, new_db) = file_parser(source).unwrap();
        assert_eq!((tail, idx), ("", idx));
        db.extend(new_db.into_iter());
    }

    db
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::sandbox::ai::{parser::named_tree_parser, Instruction, StackItem};

    use super::*;
/*
    "self"
    Some(self_id)
    self_id, true
    self_id
    self_id, self_id
    self_id, Some(hp)
    self_id, hp, true
    self_id, hp, 50
    self_id, true
    self_id, "house"
    self_id, Some(house_id)
    self_id, house_id, true
    self_id, house_id, house_id,
    self_id, house_id, Some(house_coord)
    self_id, house_id, house_coord, true 
    self_id, house_id, Success
    self_id, house_id, Success, Success
    self_id, house_id, true | if use
    self_id, Success
*/
    #[test]
    fn hermit_test() {
        let input = "dont_need_to_eat = forth {
    lit(\"self\")
    get_energy
    some_int
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
                    Instruction::ForthSomeInt,
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

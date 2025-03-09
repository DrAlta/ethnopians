use crate::sandbox::ai::{parser::file_parser, TreePool};
macro_rules! foofoo {
    // `$x` followed by at least one `$y,`
    ($($y:expr,)+) => (
        // Call `find_min!` on the tail `$y`
        [
            $((stringify!($y), $y),)+
        ]
    )
}

pub fn get_hermit_behavior_task() -> TreePool {
    let root = {
        r#"hermit = seq{
    sat_hunger,
    sat_sleep,
    have_house,
    have_garden,
    harvest_veg,
    plant_vegs
};
take_entity = forth{
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

};
take_all = forth{
    pop_last
    some_entity_id
    if
        take_entity
        lit(Success)
        eq
        if
            jump(take_all)
            return
        then
    then
    lit(False)
    eq
    if
        lit(Success)
        return
    then
    lit(Failure)
    return
}"#
    };
    /* in have_garden a gardern is (spacing * width) by (space * hiegth) plot with nothing but vegs and agents in it.
    first we try placing it at
         `my_home_location + home_height - ((spacing * width) / 2)`(x: -10, y: 5 )
    then `my_home_location + home_width - ((spacing * height) / 2)`(x: 5, y: -10 )
    then `my_home_location - (home_width + width * spacing) - ((spacing * height) / 2)`(x: -25, y: -10)
    then `my_home_location - home_height - ((spacing * width) / 2)`(x: -10, y -5 )
    */
    // Stuff used in tasks defined in root
    // hermit was defined in root so we define the words used in hermit here
    let hermit_defs = {
        r#"sat_hunger = selector{
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
        dup
        get_hp
        some_int
        if
            lit(50)
            lt
            if
                get_location
                some_coord
                if
                    go_to
                    return
                then



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
    drop
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
have_garden = forth {
    lit("garden_location")
    get_blackboard
    some_coord
    if
        drop
        lit(Success)
        return
    then
    drop
    get_my_home_location
    some_coord
    if
        dup
        lit(x: -10, y: 5)
        add
        dup
        check_if_clear_for_garden
        if
            set_garden
            drop
            lit(Success)
            return
        then
        drop
        
        dup
        lit(x: 5, y: -10)
        add
        dup
        check_if_clear_for_garden
        if
            set_garden
            drop
            lit(Success)
            return
        then
        drop

        dup
        lit(x: -25, y: -10)
        add
        dup
        check_if_clear_for_garden
        if
            set_garden
            drop
            lit(Success)
            return
        then
        drop

        dup
        lit(x: -10, y: -25)
        add
        dup
        check_if_clear_for_garden
        if
            set_garden
            drop
            lit(Success)
            return
        then
        drop





        dup
        lit(x: -10, y: 5)
        add
        dup
        clear_for_garden
        if
            set_garden
            drop
            lit(Success)
            return
        then
        drop
        
        dup
        lit(x: 5, y: -10)
        add
        dup
        clear_for_garden
        if
            set_garden
            drop
            lit(Success)
            return
        then
        drop

        dup
        lit(x: -25, y: -10)
        add
        dup
        clear_for_garden
        if
            set_garden
            drop
            lit(Success)
            return
        then
        drop

        dup
        lit(x: -10, y: -25)
        add
        dup
        clear_for_garden
        if
            set_garden
            drop
            lit(Success)
            return
        then
        drop
        drop
    then
    lit(Failure)
    return
};
harvest_veg = forth{
    lit("garden_location")
    some_coord
    if
        lit(x: 20, y: 20)
        get_entities
        lit("veggies")
        retain_entities_of_type
        take_all
        swap
        drop
        return
    then
    drop
    lit(Failure)
    return
};
plant_vegs = forth {
    lit(12)
    have_n_seed
    drop
    lit("garden_location")
    get_blackboard
    some_coord
    if
        lit(True)
        swap
        dup
        plant_row
        plant_vegs_02
        dup
        lit(x:0, y: 5)
        add
        plant_row
        plant_vegs_02
        dup
        lit(x:0, y: 5)
        add
        plant_row
        plant_vegs_02
        dup
        lit(x:0, y: 5)
        add
        plant_row
        plant_vegs_02
        dup
        lit(x:0, y: 5)
        add
        plant_row
        plant_vegs_02
        drop
        if
            lit(Success)
            return
        then
    then
    lit(Failure)
    return
}"#
    };
    // vvv tasks used in tasks defined in hermit_defs
    let sat_hunger_defs = {
        r#"dont_need_to_eat = forth {
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
    drop
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
                    take_entity
                    return
                then
            then
            lit(Failure)
            return
        }
    }
}"#
    };
    let have_house_defs = {
        r#"is_house_in_range = forth{
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
}"#
    };
    // have_garden check is their is a location of a garden in the blackboard,
    // if not then it finds the agent's house
    // then checks south, then west the east the north of the house for a spot that
    // is clear of everything but veg and agents,
    // when it finds one it sets that to the garden,
    // else it tries to clear the spots in th same order
    // and when it succeeds it sets the cleared spot to be the garden
    let have_garden_defs = {
        r#"get_my_home_location = forth {
    lit("self")
    get_blackboard
    some_entity_id
    if
        dup
        get_location
        some_coord
        if
            lit("house")
            find_nearest
            some_entity_id
            if
                get_location
                return
            then
        then
    then
    drop
    lit(false)
    return
};
check_if_clear_for_garden = forth{
    lit(x: 20, y: 20)
    get_entities
    lit("veggie")
    remove_entities_of_type
    lit("agent")
    remove_entities_of_type
    is_empty
};
clear_for_garden = forth{
    lit(x: 20, y: 20)
    get_entities
    take_all
    swap
    drop
    return
};
set_garden = forth {
        lit("garden_location")
        set_blackboard
}"#
    }; // todo need to add set_blackboard and defer_blackboard instructions
    let plant_vegs_defs = {
        r#"have_n_seed = forth{
    dup
    lit("seed")
    inventory_have_ge
    if
        drop
        lit(Success)
        return
    then
    split_veg_to_seed
    lit(Success)
    not_true
    if
        lit(Failure)
        return
    then
    jump(have_n_seed)
};
plant_row = forth{
    dup
    plant_seed
    lit(0)
    plant_row_02
};
plant_vegs_02 = forth{
    lit(Success)
    eq
    rot
    or
    swap
    return
}"#
    };
    // vvv tasks used in taks defined in have_house_defs
    let have_2_wood_defs = {
        r#"have_2_wood_02 = sel{
    inventory_have_ge(wood, 2),
    have_axe,
    go_to_tree,
    use(axe, tree)
}"#
    };
    // vvv tasks used in tasks defined in have_2_wood_defs
    let have_2_wood_02_defs = {
        r#"have_axe = sel{
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
}"#
    };
    // vvv tasks used in tasked defined in have_2_wood_02_defs
    let have_axe_defs = {
        r#"have_knife = sel{
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
}"#
    };
    let have_knife_defs = {
        r#"have_2_stone = seq{
    have_2_stone_02,
    have_2_stone_02
}"#
    };
    let have_2_stone_defs = {
        r#"have_2_stone_02 = sel{
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
}"#
    };
    //defs of words used in words use plant_vegs
    let have_n_seeds_defs = {
        r#"split_veg_to_seed = forth{
    have_knife
    lit(Success)
    not_true
    if
        lit(Failure)
        return
    then
    lit(1)
    lit("veggie")
    inventory_have_ge
    not_true
    if
        lit("veggie")
        find_nearest
        some_entity_id
        if
            dup
            get_location
            some_coord
            if
                go_to
                take
                lit(Success)
                eq
                not_true
                if
                    lit(Failure)
                    return
                then
                lit("seed")
                find_in_inventory
                some_entity_id
                if
                    use
                    return
                then
                drop
            then
        then
    then
    lit(Failure)
    return
}"#
    };
    /*
    coord_a int_a
    coord_a int_a int_a
    coord_a int_a true
    int_a coord_a [5,0]
    int_a coord_b
    int_a coord_b coord_b
    int_a coord_b success
    int_a coord_b success success
    int_a coord_b true
    */
    let plant_row_defs = {
        r#"plant_row_02 /*(Coord Int -- Succes/Failure) plants seed  at Int multiples of  [x:5,Y:0] from coord  */= forth{
    dup
    lit(4)
    lt
    if
        swap
        lit(x: 5, y: 0)
        add
        dup
        plant_seed
        lit(Success)
        eq
        if
            swap
            lit(1)
            sub
            jump(plant_row_02)
        then
        lit(Failure)
        return
    then
    drop
    lit(Success)
    return
}"#
    };

    let plant_row_02_defs = {
        r#"plant_seed /* (coord -- Success/Failure ) plants a seed at coord*/= forth{
    lit("seed")
    find_in_inventory
    some_entity_id
    if
        swap
        go_to
        lit(Success)
        eq
        if
            plant
            return
        then
    then
    drop
    lit(Failure)
    return
}"#
    };

    // end tasks used in tasked defined in have_2_wood_02_defs

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

    for (idx, source) in foofoo![
        //test,
        hermit_defs,
        sat_hunger_defs,
        have_house_defs,
        have_garden_defs,
        plant_vegs_defs,
        have_2_wood_defs,
        have_2_wood_02_defs,
        have_axe_defs,
        have_knife_defs,
        have_2_stone_defs,
        have_n_seeds_defs,
        plant_row_defs,
        plant_row_02_defs,
    ]
    .into_iter()
    //    .enumerate()
    {
        let (tail, new_db) = file_parser(source).expect("{idx} didn't parse");
        assert_eq!((tail, idx), ("", idx));
        db.extend(new_db.into_iter());
    }

    //    logy!("debug", "{:?}", db.get("check_if_clear_for_garden"));
    db
}

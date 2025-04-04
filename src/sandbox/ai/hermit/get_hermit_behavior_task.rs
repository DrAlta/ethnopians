use crate::sandbox::ai::{
    hermit::{
        HAVE_2_STONE_DEFS, HAVE_2_WOOD_02_DEFS, HAVE_2_WOOD_DEFS, HAVE_AXE_DEFS, HAVE_GARDEN_DEFS,
        HAVE_HOUSE_DEFS, HAVE_KNIFE_DEFS, HAVE_N_SEEDS_DEFS, HERMIT_DEFS, PLANT_ROW_02_DEFS,
        PLANT_ROW_DEFS, PLANT_VEGS_DEFS, SAT_HUNGER_DEFS,
    },
    parser::file_parser,
    TaskPool,
};
macro_rules! foofoo {
    // `$x` followed by at least one `$y,`
    ($($y:expr,)+) => (
        // Call `find_min!` on the tail `$y`
        [
            $((stringify!($y), $y),)+
        ]
    )
}

pub fn get_hermit_behavior_task() -> TaskPool {
    let root = {
        r#"hermit = seq{
    sat_hunger,
    have_house,
    sat_sleep,
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
take_all /* (table -- bool) */ = forth{
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
    swap
    drop
    return
}"#
    };

    let (tail1, mut db) = file_parser(root).unwrap();
    assert_eq!(tail1, "");

    for (idx, source) in foofoo![
        //test,
        HERMIT_DEFS,
        SAT_HUNGER_DEFS,
        HAVE_HOUSE_DEFS,
        HAVE_GARDEN_DEFS,
        PLANT_VEGS_DEFS,
        HAVE_2_WOOD_DEFS,
        HAVE_2_WOOD_02_DEFS,
        HAVE_AXE_DEFS,
        HAVE_KNIFE_DEFS,
        HAVE_2_STONE_DEFS,
        HAVE_N_SEEDS_DEFS,
        PLANT_ROW_DEFS,
        PLANT_ROW_02_DEFS,
    ]
    .into_iter()
    {
        let (tail, new_db) = file_parser(source).expect("{idx} didn't parse");
        assert_eq!((tail, idx), ("", idx));
        db.extend(new_db.into_iter());
    }
    
    db.extend(TaskPool::core());

    db
}

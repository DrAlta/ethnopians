use crate::sandbox::bt::parser::parse_file;

use super::TreePool;

pub fn get_hermit_behavoir_tree() -> TreePool {
    let source = r#"
have_2_stone_2 = sel{
    inventory_have_ge(stone, 2),
    seq{
        go_to_stone,
        take_stone
    }
};
have_2_stone = seq{
    have_2_stone_2,
    have_2_stone_2
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
have_2_wood_2 = sel{
    inventory_have_ge(wood, 2),
    have_axe,
    go_to_tree,
    use(axe, tree)
};
have_2_wood =seq{
    have_2_wood_2,
    have_2_wood_2
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
    seq{
        selector{
            inventory_have_ge(veg, 1),
            get_veg
        },
        eat(veg)
    }
}
    "#;
    let (tail, db) = parse_file(source).unwrap();
    assert_eq!(tail, "");
    db
}
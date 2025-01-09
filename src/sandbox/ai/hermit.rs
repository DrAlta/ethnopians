use crate::sandbox::bt::{TreePool, parser::file_parser};

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
    seq{
        selector{
            inventory_have_ge(veg, 1),
            get_veg
        },
        eat(veg)
    }
}
    "#;
    let (tail, db) = file_parser(source).unwrap();
    assert_eq!(tail, "");
    db
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    pub fn check_for_missing_threads_in_hermit_ai() {
        let bt = get_hermit_behavoir_tree();
        let mut missing = HashMap::new();
        for(thread, i) in &bt {
            let x = i.missing_threads_used(&bt);
            if !x.is_empty() {
                missing.insert(thread, x);
            }
        }
        for (a,b) in &missing {
            println!("{a}:{b:?}");
        }
        assert_eq!(
            missing,
            HashMap::new()
        )
    }
}

use sapielib::combat::{Creature, calc_weapon};

fn main() {
    println!("Hello, world!");
    let creature = Creature{ combat_st: 1.0, load_st: 1.0 };
    let _ = (
        creature.get_combat_st(),
        creature.get_load_st()
    );
    let _ = calc_weapon(1.0, 1.0);

}

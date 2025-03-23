mod get_hermit_behavior_task;
pub use get_hermit_behavior_task::get_hermit_behavior_task;

pub const HAVE_2_STONE_DEFS: &str = include_str!("tasks/have_2_stone_defs.tasks");
pub const HAVE_2_WOOD_02_DEFS: &str = include_str!("tasks/have_2_wood_02_defs.tasks");
pub const HAVE_2_WOOD_DEFS: &str = include_str!("tasks/have_2_wood_defs.tasks");
pub const HAVE_AXE_DEFS: &str = include_str!("tasks/have_axe_defs.tasks");
pub const HAVE_GARDEN_DEFS: &str = include_str!("tasks/have_garden_defs.tasks");
pub const HAVE_HOUSE_DEFS: &str = include_str!("tasks/have_house_defs.tasks");
pub const HAVE_KNIFE_DEFS: &str = include_str!("tasks/have_knife_defs.tasks");
pub const HAVE_N_SEEDS_DEFS: &str = include_str!("tasks/have_n_seeds_defs.tasks");
pub const HERMIT_DEFS: &str = include_str!("tasks/hermit_defs.tasks");
pub const PLANT_ROW_02_DEFS: &str = include_str!("tasks/plant_row_02_defs.tasks");
pub const PLANT_ROW_DEFS: &str = include_str!("tasks/plant_row_defs.tasks");
pub const PLANT_VEGS_DEFS: &str = include_str!("tasks/plant_vegs_defs.tasks");
pub const SAT_HUNGER_DEFS: &str = include_str!("tasks/sat_hunger_defs.tasks");

#[cfg(test)]
mod tests;

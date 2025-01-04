use qol::logy;

use crate::sandbox::{process_movement, Command, World};

impl World {
    pub fn step_sim(&mut self, max_step: f32, time_step: f32, commands: Vec<Command>) {
        self.execute_commands(commands);
        let (movemevt_return, _) = process_movement(max_step, time_step, self);
        match movemevt_return {
            crate::sandbox::Return::ActionInvalid(_x) => {
                logy!("warn", "{_x}");
            }
            crate::sandbox::Return::Commands(movement_commands) => {
                self.execute_commands(movement_commands);
            }
        }
    }
}

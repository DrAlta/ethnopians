use crate::sandbox::{Command, World};

impl World {
    pub fn execute_command(&mut self, command: Command) {
        match command {
            Command::AddItem { item, loc } => {
                let new_id = self.get_new_object_id();
                self.highest_id = new_id.clone();
                self.r#type.insert(new_id.clone(), item);
                self.locations.insert(new_id.clone(), loc);
            }
            Command::Remove(object_id) => {
                self.locations.remove(&object_id);
                self.energy.remove(&object_id);
                self.hp.remove(&object_id);
                self.sizes.remove(&object_id);
                self.r#type.remove(&object_id);
            }
            Command::Damage { agent_id, ammount } => {
                if let Some(hp) = self.hp.get_mut(&agent_id) {
                    *hp -= ammount;
                }
            }
            Command::Rest { agent_id, ammount } => {
                if let Some(energy) = self.energy.get_mut(&agent_id) {
                    *energy += ammount;
                }
            }
            Command::Heal { agent_id, ammount } => {
                if let Some(hp) = self.hp.get_mut(&agent_id) {
                    *hp += ammount;
                }
            }
            Command::SetLocation { agent_id, loc } => {
                self.locations.insert(agent_id, loc);
            }
        }
    }
    pub fn execute_commands(&mut self, commands: Vec<Command>) {
        for command in commands {
            self.execute_command(command);
        }
    }
}

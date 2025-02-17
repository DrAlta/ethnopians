use bevy::prelude::*;

pub trait Dispatch {
    fn dispatch(&self, commands: &mut Commands);
}

impl<T: Dispatch> Dispatch for Vec<T> {
    fn dispatch(&self, commands: &mut Commands) {
        self.iter().for_each(|x| x.dispatch(commands));
    }
}

use bevy::prelude::*;

pub trait Dispatch {
    fn dispatch(self, commands: &mut Commands);
}

impl<'a, T: Clone + Dispatch> Dispatch for &'a Vec<T>
{
    fn dispatch(self, commands: &mut Commands) {
        self.iter().for_each(|x| x.clone().dispatch(commands));
    }
}
